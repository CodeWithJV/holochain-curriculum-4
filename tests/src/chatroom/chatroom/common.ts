import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleRoom(cell: CallableCell, partialRoom = {}) {
    return {
        ...{
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          creator: cell.cell_id[1],
        },
        ...partialRoom
    };
}

export async function createRoom(cell: CallableCell, room = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "chatroom",
      fn_name: "create_room",
      payload: room || await sampleRoom(cell),
    });
}



export async function sampleMessage(cell: CallableCell, partialMessage = {}) {
    return {
        ...{
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          creator: cell.cell_id[1],
	  timestamp: 1674053334548000,
          room_hash: (await createRoom(cell)).signed_action.hashed.hash,
        },
        ...partialMessage
    };
}

export async function createMessage(cell: CallableCell, message = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "chatroom",
      fn_name: "create_message",
      payload: message || await sampleMessage(cell),
    });
}

