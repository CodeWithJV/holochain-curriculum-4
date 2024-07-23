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
  import type { Message } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let content!: string

  export let creator!: AgentPubKey

  export let timestamp!: number

  export let roomHash!: ActionHash

  let errorSnackbar: Snackbar

  $: content, creator, timestamp, roomHash
  $: isMessageValid = true

  onMount(() => {
    if (content === undefined) {
      throw new Error(
        `The content input is required for the CreateMessage element`
      )
    }
    if (creator === undefined) {
      throw new Error(
        `The creator input is required for the CreateMessage element`
      )
    }
    if (timestamp === undefined) {
      throw new Error(
        `The timestamp input is required for the CreateMessage element`
      )
    }
    if (roomHash === undefined) {
      throw new Error(
        `The roomHash input is required for the CreateMessage element`
      )
    }
  })

  async function createMessage() {
    const messageEntry: Message = {
      content: content!,
      creator: creator!,
      timestamp: timestamp!,
      room_hash: roomHash!,
    }

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'create_message',
        payload: messageEntry,
      })
      dispatch('message-created', {
        messageHash: record.signed_action.hashed.hash,
      })
    } catch (e) {
      errorSnackbar.labelText = `Error creating the message: ${error}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Message</span>

  <mwc-button
    raised
    label="Create Message"
    disabled={!isMessageValid}
    on:click={() => createMessage()}
  ></mwc-button>
</div>
