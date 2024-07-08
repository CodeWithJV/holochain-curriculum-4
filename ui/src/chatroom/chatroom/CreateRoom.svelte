<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type {
    AppClient,
    Record,
    EntryHash,
    AgentPubKey,
    ActionHash,
    DnaHash,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Room } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let name!: string

  export let creator!: AgentPubKey

  let errorSnackbar: Snackbar

  $: name, creator
  $: isRoomValid = true

  onMount(() => {
    if (name === undefined) {
      throw new Error(`The name input is required for the CreateRoom element`)
    }
    if (creator === undefined) {
      throw new Error(
        `The creator input is required for the CreateRoom element`
      )
    }
  })

  async function createRoom() {
    const roomEntry: Room = {
      name: name!,
      creator: creator!,
    }

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'create_room',
        payload: roomEntry,
      })
      dispatch('room-created', { roomHash: record.signed_action.hashed.hash })
    } catch (e) {
      errorSnackbar.labelText = `Error creating the room: ${error}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Room</span>

  <mwc-button
    raised
    label="Create Room"
    disabled={!isRoomValid}
    on:click={() => createRoom()}
  ></mwc-button>
</div>
