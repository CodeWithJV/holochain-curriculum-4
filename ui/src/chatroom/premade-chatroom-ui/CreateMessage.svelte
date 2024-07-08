<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type {
    AppClient,
    Record,
    AgentPubKey,
    ActionHash,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Message } from '../../types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-textarea'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let creator!: AgentPubKey
  export let roomHash!: ActionHash

  let content: string = ''
  let timestamp: number = Date.now()

  let errorSnackbar: Snackbar

  $: content, creator, timestamp, roomHash
  $: isMessageValid = true && content !== '' && true

  onMount(() => {
    if (creator === undefined) {
      throw new Error(
        `The creator input is required for the CreateMessage element`
      )
    }
  })

  async function createMessage() {
    const messageEntry: Message = {
      content: content!,
      creator: creator!,
      timestamp: timestamp!,
      room_hash: roomHash,
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

<mwc-snackbar bind:this={errorSnackbar} leading></mwc-snackbar>
<div style="display: flex; flex-direction: column; gap: 16px;">
  <mwc-textarea
    outlined
    label="Message"
    value={content}
    on:input={(e) => {
      content = e.target.value
    }}
    required
  ></mwc-textarea>

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <mwc-button
    raised
    label="Create Message"
    disabled={!isMessageValid}
    on:click={() => createMessage()}
  ></mwc-button>
</div>
