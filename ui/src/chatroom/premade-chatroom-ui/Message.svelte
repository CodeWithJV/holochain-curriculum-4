<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import { decode } from '@msgpack/msgpack'
  import {
    type Record,
    type ActionHash,
    type AppClient,
    encodeHashToBase64,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Message } from '../../types'
  import '@material/mwc-circular-progress'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-snackbar'
  import '@material/mwc-icon-button'

  const dispatch = createEventDispatcher()

  export let messageHash: ActionHash

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let loading: boolean
  let error: any = undefined

  let record: Record | undefined
  let message: Message | undefined
  let messageCreatorSliced: string
  let messageCreator: string

  let myPubKey: string

  let editing = false

  let errorSnackbar: Snackbar

  $: editing,
    error,
    loading,
    record,
    message,
    messageCreatorSliced,
    messageCreator,
    myPubKey

  onMount(async () => {
    myPubKey = encodeHashToBase64(client.myPubKey)
    if (messageHash === undefined) {
      throw new Error(
        `The messageHash input is required for the MessageDetail element`
      )
    }
    await fetchMessage()
  })

  async function fetchMessage() {
    loading = true

    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'get_message',
        payload: messageHash,
      })
      if (record) {
        message = decode((record.entry as any).Present.entry) as Message
        messageCreator = encodeHashToBase64(message?.creator)
        messageCreatorSliced = messageCreator.slice(0, 7)
      }
    } catch (e) {
      error = e
    }

    loading = false
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>

{#if loading}
  <mwc-circular-progress indeterminate></mwc-circular-progress>
{:else if error}
  <span>Error fetching the message: {error}</span>
{:else if messageCreator === myPubKey}
  <div
    style="text-align: start; display: flex; flex-direction: row; padding:10px; background-color: #c2a5d0; border-radius: 10px; width: 100%; max-width: 40%; margin-left: auto;"
  >
    <span style="margin-right: 4px"><strong>You:</strong></span>
    <span style="white-space: pre-line">{message?.content}</span>
  </div>
{:else}
  <div
    style="display: flex; flex-direction: row; padding:10px; background-color: #d4bbff; border-radius: 10px; width: 100%; max-width: 40%;"
  >
    <span style="margin-right: 4px"
      ><strong>{messageCreatorSliced}:</strong></span
    >
    <span style="white-space: pre-line; text-align: start;"
      >{message?.content}</span
    >
  </div>
{/if}
