import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
import {
  NewEntryAction,
  ActionHash,
  Record,
  Link,
  CreateLink,
  DeleteLink,
  SignedActionHashed,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash
} from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createMessage, sampleMessage } from './common.js';

test('create Message', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/c-4.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Message
    const record: Record = await createMessage(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read Message', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/c-4.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleMessage(alice.cells[0]);

    // Alice creates a Message
    const record: Record = await createMessage(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Message
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "chatroom",
      fn_name: "get_message",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the Creators for the new Message
    let linksToCreators: Link[] = await bob.cells[0].callZome({
      zome_name: "chatroom",
      fn_name: "get_messages_for_creator",
      payload: sample.creator
    });
    assert.equal(linksToCreators.length, 1);
    assert.deepEqual(linksToCreators[0].target, record.signed_action.hashed.hash);
    // Bob gets the Rooms for the new Message
    let linksToRooms: Link[] = await bob.cells[0].callZome({
      zome_name: "chatroom",
      fn_name: "get_messages_for_room",
      payload: sample.room_hash
    });
    assert.equal(linksToRooms.length, 1);
    assert.deepEqual(linksToRooms[0].target, record.signed_action.hashed.hash);
  });
});


