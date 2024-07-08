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
  export let created: boolean = false
  export let openChatRoom: (roomHash: ActionHash) => void

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let loading: boolean
  let error: any = undefined

  let record: Record | undefined
  let room: Room | undefined

  let errorSnackbar: Snackbar

  $: error, loading, record, room, created

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
    <div style="display: flex; flex-direction: row">
      <mwc-button
        raised
        label="Chat"
        style="margin-top: auto; margin-bottom: auto;"
        on:click={openChatRoom(roomHash)}
      ></mwc-button>
    </div>
  </div>
{/if}
