use hdk::prelude::*;
use chatroom_integrity::*;

#[hdk_extern]
pub fn create_room(room: Room) -> ExternResult<Record> {
    let room_hash = create_entry(&EntryTypes::Room(room.clone()))?;
    create_link(room.creator.clone(), room_hash.clone(), LinkTypes::CreatorToRooms, ())?;

    create_link(room.creator.clone(), room_hash.clone(), LinkTypes::MemberToRooms, ())?;
    create_link(room_hash.clone(), room.creator.clone(), LinkTypes::RoomToMembers, ())?;

    let record = get(room_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly created Room".to_string()))
    )?;
    let path = Path::from("all_rooms");
    create_link(path.path_entry_hash()?, room_hash.clone(), LinkTypes::AllRooms, ())?;
    Ok(record)
}

#[hdk_extern]
pub fn get_room(room_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(room_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string())))
        }
    }
}

#[hdk_extern]
pub fn get_rooms_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(creator, LinkTypes::CreatorToRooms)?.build())
}
