
<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Link, ActionHash, EntryHash, AppClient, Record, AgentPubKey, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Message, ChatroomSignal } from './types';
import MessageDetail from './MessageDetail.svelte';

export let roomHash: ActionHash;

let client: AppClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined = [];

let loading: boolean;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (roomHash === undefined) {
    throw new Error(`The roomHash input is required for the MessagesForRoom element`);
  }

  await fetchMessages();

  client.on('signal', async signal => {
    if (signal.zome_name !== 'chatroom') return;
    const payload = signal.payload as ChatroomSignal;
    if (!(payload.type === 'EntryCreated' && payload.app_entry.type === 'Message')) return;
    await fetchMessages();
  });
});

async function fetchMessages() {
  loading = true;
  try {
    const links: Array<Link> = await client.callZome({
      cap_secret: null,
      role_name: 'chatroom',
      zome_name: 'chatroom',
      fn_name: 'get_messages_for_room',
      payload: roomHash
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching messages: ${error.data.data}.</span>
{:else if hashes.length === 0}
<span>No messages found for this room.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <MessageDetail messageHash={hash} on:message-deleted={fetchMessages}></MessageDetail>
    </div>
  {/each}
</div>
{/if}
