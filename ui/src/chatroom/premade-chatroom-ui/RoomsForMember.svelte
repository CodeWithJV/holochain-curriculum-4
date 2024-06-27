<script lang="ts">
  import { onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import type { ActionHash, AgentPubKey, AppClient } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import RoomDetail from './SearchRoomsListing.svelte'
  import type { ChatroomSignal } from '../../types'

  export let member: AgentPubKey

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let hashes: Array<ActionHash> | undefined

  let loading = true
  let error: any = undefined

  $: hashes, loading, error

  onMount(async () => {
    if (member === undefined) {
      throw new Error(
        `The member input is required for the RoomsForMember element`
      )
    }

    try {
      const links = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'get_rooms_for_member',
        payload: member,
      })
      hashes = links.map((l) => l.target)
    } catch (e) {
      error = e
    }
    loading = false

    client.on('signal', (signal) => {
      if (signal.zome_name !== 'chatroom') return
      const payload = signal.payload as ChatroomSignal
      if (payload.type !== 'LinkCreated') return
      if (payload.link_type !== 'MemberToRooms') return

      hashes = [...hashes, payload.action.hashed.content.target_address]
    })
  })
</script>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <span>Error fetching rooms: {error.data}.</span>
{:else if hashes.length === 0}
  <span>No rooms found for this member.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        <RoomDetail roomHash={hash}></RoomDetail>
      </div>
    {/each}
  </div>
{/if}
