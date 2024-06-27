pub mod member_to_rooms;
pub mod message;
pub mod all_rooms;
pub mod room;
use hdk::prelude::*;
use chatroom_integrity::*;

use crate::member_to_rooms::get_members_for_room;

pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
    functions.insert((ZomeName::from("chatroom"), FunctionName::from("recv_remote_signal")));
    create_cap_grant(CapGrantEntry {
        tag: "recv_remote_signal_unrestricted".into(),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: Signal) -> ExternResult<()> {
    debug!("Received signal!");
    emit_signal(signal)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    LinkCreated {
        action: SignedActionHashed,
        link_type: LinkTypes,
    },
    LinkDeleted {
        action: SignedActionHashed,
        create_link_action: SignedActionHashed,
        link_type: LinkTypes,
    },
    EntryCreated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
    },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted {
        action: SignedActionHashed,
        original_app_entry: EntryTypes,
    },
}

#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}

fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::CreateLink(create_link) => {
            if
                let Ok(Some(link_type)) = LinkTypes::from_type(
                    create_link.zome_index,
                    create_link.link_type
                )
            {
                emit_signal(Signal::LinkCreated {
                    action,
                    link_type,
                })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(delete_link.link_add_address.clone(), GetOptions::default())?.ok_or(
                wasm_error!(WasmErrorInner::Guest("Failed to fetch CreateLink action".to_string()))
            )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if
                        let Ok(Some(link_type)) = LinkTypes::from_type(
                            create_link.zome_index,
                            create_link.link_type
                        )
                    {
                        emit_signal(Signal::LinkDeleted {
                            action,
                            link_type,
                            create_link_action: record.signed_action.clone(),
                        })?;
                    }
                    Ok(())
                }
                _ => {
                    Err(wasm_error!(WasmErrorInner::Guest("Create Link should exist".to_string())))
                }
            }
        }
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                let new_signal = Signal::EntryCreated {
                    app_entry,
                    action: action.clone(),
                };
                emit_signal(&new_signal)?;

                // If the create action is of type Message
                if
                    action.action().entry_type().unwrap().clone() ==
                    UnitEntryTypes::Message.try_into()?
                {
                    // Get the entry off the create action

                    let record = get(action.hashed.hash.clone(), GetOptions::default())?.unwrap();

                    let message = record.entry().to_app_option::<Message>().unwrap().unwrap();

                    // Get the room hash from the entry
                    let room_hash = message.room_hash;

                    // Get the members for the room using the room hash
                    let members: Vec<AgentPubKey> = get_members_for_room(room_hash.clone())?
                        .into_iter()
                        .map(|link| {
                            AgentPubKey::try_from(link.target)
                                .map_err(|_| {
                                    wasm_error!(
                                        WasmErrorInner::Guest(
                                            String::from("Could not convert link to agent pub key")
                                        )
                                    )
                                })
                                .unwrap()
                        })
                        // .filter(|agent| *agent != agent_info().unwrap().agent_latest_pubkey)
                        .collect();

                    let length = members.len();

                    debug!("len: {length}, members: {:?}", members);

                    let _ = send_remote_signal(new_signal, members);
                }
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if
                    let Ok(Some(original_app_entry)) = get_entry_for_action(
                        &update.original_action_address
                    )
                {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry)) = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef { zome_index, entry_index, .. })) => {
            (zome_index, entry_index)
        }
        _ => {
            return Ok(None);
        }
    };
    EntryTypes::deserialize_from_type(*zome_index, *entry_index, entry)
}
