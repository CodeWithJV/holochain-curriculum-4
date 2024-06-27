use hdk::prelude::*;
use chatroom_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRoomForMemberInput {
    pub base_member: AgentPubKey,
    pub target_room_hash: ActionHash,
}

#[hdk_extern]
pub fn add_room_for_member(input: AddRoomForMemberInput) -> ExternResult<()> {
    create_link(
        input.base_member.clone(),
        input.target_room_hash.clone(),
        LinkTypes::MemberToRooms,
        ()
    )?;
    create_link(input.target_room_hash, input.base_member, LinkTypes::RoomToMembers, ())?;
    Ok(())
}

#[hdk_extern]
pub fn get_rooms_for_member(member: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(member, LinkTypes::MemberToRooms)?.build())
}

#[hdk_extern]
pub fn get_members_for_room(room_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(room_hash, LinkTypes::RoomToMembers)?.build())
}

#[hdk_extern]
pub fn get_not_joined_rooms_for_member(member: AgentPubKey) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_rooms");
    let all_rooms = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllRooms)?.build()
    )?;
    let joined_rooms = get_links(
        GetLinksInputBuilder::try_new(member, LinkTypes::MemberToRooms)?.build()
    )?;
    let joined_rooms_set: HashSet<_> = joined_rooms
        .into_iter()
        .map(|r| r.target)
        .collect();
    let not_joined_rooms = all_rooms
        .into_iter()
        .filter(|room| !joined_rooms_set.contains(&room.target))
        .collect::<Vec<_>>();
    Ok(not_joined_rooms)
}
