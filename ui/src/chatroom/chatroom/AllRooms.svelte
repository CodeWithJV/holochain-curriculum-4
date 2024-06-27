<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import RoomDetail from './RoomDetail.svelte';
import type { ChatroomSignal } from './types';


let client: AppClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchRooms();
  client.on('signal', signal => {
    if (signal.zome_name !== 'chatroom') return;
    const payload = signal.payload as ChatroomSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Room') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchRooms() {
  try {
    const links = await client.callZome({
      cap_secret: null,
      role_name: 'chatroom',
      zome_name: 'chatroom',
      fn_name: 'get_all_rooms',
      payload: null,
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the rooms: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No rooms found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <RoomDetail roomHash={hash}  on:room-deleted={() => fetchRooms()}></RoomDetail>
    </div>
  {/each}
</div>
{/if}

