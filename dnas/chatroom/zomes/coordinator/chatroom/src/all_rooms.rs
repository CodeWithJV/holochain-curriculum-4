use hdk::prelude::*;
use chatroom_integrity::*;

#[hdk_extern]
pub fn get_all_rooms() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_rooms");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllRooms)?
            .build(),
    )
}
