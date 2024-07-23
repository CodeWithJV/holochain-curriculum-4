<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import { decode } from '@msgpack/msgpack'
  import type { Record, ActionHash, AppClient } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Page, Room } from '../../types'
  import '@material/mwc-circular-progress'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-snackbar'
  import '@material/mwc-icon-button'

  const dispatch = createEventDispatcher()

  export let roomHash: ActionHash

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let loading: boolean
  let error: any = undefined

  let record: Record | undefined
  let room: Room | undefined

  let editing = false

  let errorSnackbar: Snackbar

  $: editing, error, loading, record, room

  onMount(async () => {
    if (roomHash === undefined) {
      throw new Error(
        `The roomHash input is required for the RoomDetail element`
      )
    }
    await fetchRoom()
  })

  async function fetchRoom() {
    loading = true

    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'get_room',
        payload: roomHash,
      })
      if (record) {
        room = decode((record.entry as any).Present.entry) as Room
      }
    } catch (e) {
      error = e
    }

    loading = false
  }

  async function joinRoom() {
    try {
      await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'add_room_for_member',
        payload: {
          base_member: client.myPubKey,
          target_room_hash: roomHash,
        },
      })
      dispatch('room-joined', { roomHash: roomHash })
    } catch (e: any) {
      errorSnackbar.labelText = `Error joining the room: ${error}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <span>Error fetching the room: {error}</span>
{:else}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    style="display: flex; flex-direction: row; justify-content: space-between;"
  >
    <span style="white-space: pre-line; margin-top: auto; margin-bottom: auto;"
      ><strong>{room?.name}</strong></span
    >
    <mwc-button raised label="Join" on:click={joinRoom}></mwc-button>
  </div>
{/if}
