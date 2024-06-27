<script lang="ts">
  import { onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import type { ActionHash, AppClient, Link } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import Listing from './YourRoomListing.svelte'
  import type { ChatroomSignal } from '../../types'

  export let openChatRoom: (roomHash: ActionHash) => void

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let hashes: Array<ActionHash> | undefined
  let loading = true
  let error: any = undefined

  $: hashes, loading, error

  onMount(async () => {
    await fetchRooms()
    client.on('signal', (signal) => {
      if (signal.zome_name !== 'chatroom') return
      const payload = signal.payload as ChatroomSignal
      if (payload.type !== 'EntryCreated') return
      if (payload.app_entry.type !== 'Room') return
      hashes = [...hashes, payload.action.hashed.hash]
    })
  })

  async function fetchRooms() {
    try {
      const links = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'get_rooms_for_member',
        payload: client.myPubKey,
      })
      hashes = links.map((l) => l.target)
    } catch (e) {
      error = e
    }
    loading = false
  }
</script>

<h1>Joined Chat Rooms</h1>
{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <span>Error fetching the rooms: {error.data}.</span>
{:else if hashes.length === 0}
  <span>No rooms found.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        <Listing
          {openChatRoom}
          created={false}
          roomHash={hash}
          on:room-joined={() => fetchRooms()}
        ></Listing>
      </div>
    {/each}
  </div>
{/if}
