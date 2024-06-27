<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type { AppClient, Record, AgentPubKey } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Room } from '../../types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-textfield'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let creator!: AgentPubKey

  let name: string = ''

  let errorSnackbar: Snackbar

  $: name, creator
  $: isRoomValid = true && name !== ''

  onMount(() => {
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
      errorSnackbar.labelText = `Error creating the room: ${e.data}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Room</span>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      maxlength="20"
      label="Name"
      value={name}
      on:input={(e) => {
        name = e.target.value
      }}
      required
    ></mwc-textfield>
  </div>

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <mwc-button
    style="width: 232px; margin-left: auto; margin-right: auto;"
    raised
    label="Create Room"
    disabled={!isRoomValid}
    on:click={() => createRoom()}
  ></mwc-button>
</div>
