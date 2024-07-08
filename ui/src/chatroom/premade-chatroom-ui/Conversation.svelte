<script lang="ts">
  import Message from './Message.svelte'
  import { getContext, onMount } from 'svelte'
  import CreateMessage from './CreateMessage.svelte'
  import {
    encodeHashToBase64,
    type ActionHash,
    type AppClient,
    type Link,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { ChatroomSignal } from '../../types'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  export let roomHash!: ActionHash

  let hashes: Array<ActionHash> | undefined
  let error: any = undefined
  let loading = true
  let scrollContainer: HTMLDivElement

  $: hashes, error, loading, roomHash
  onMount(async () => {
    if (roomHash == undefined) {
      throw new Error(`roomHash is undefined`)
    }
    await fetchMessages()
    client.on('signal', (signal) => {
      if (signal.zome_name !== 'chatroom') return
      const payload = signal.payload as ChatroomSignal
      switch (payload.type) {
        case 'EntryCreated':
          if (payload.app_entry.type === 'Message')
            hashes = [...hashes, payload.action.hashed.hash]
          break
        case 'RemoteMessageCreated':
          if (
            encodeHashToBase64(payload.room_hash) ===
            encodeHashToBase64(roomHash)
          )
            addMessage(payload.action.hashed.hash)
          break

        default:
          break
      }
      return
    })
  })

  async function addMessage(message: ActionHash) {
    await new Promise((resolve) => setTimeout(resolve, 1000))
    hashes = [...hashes, message]
    setTimeout(scrollToBottom, 0)
  }

  async function fetchMessages() {
    try {
      const links: Link[] = await client.callZome({
        cap_secret: null,
        role_name: 'chatroom',
        zome_name: 'chatroom',
        fn_name: 'get_messages_for_room',
        payload: encodeHashToBase64(roomHash),
      })

      hashes = links.map((l) => l.target)
      setTimeout(scrollToBottom, 0)
    } catch (e) {
      error = e
    }
    loading = false
  }

  $: if (hashes) {
    setTimeout(scrollToBottom, 0)
  }

  function scrollToBottom() {
    if (scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight
    }
  }
</script>

<div
  style="padding: 16px; padding-top: 0; box-sizing: border-box; flex: 1; display: flex; flex-direction: column-reverse; height: 100%; max-width: 600px; width: 100%; margin-left: auto; margin-right: auto;"
>
  <CreateMessage {roomHash} creator={client.myPubKey} />
  <span style="height: 32px; display: block;"></span>
  {#if loading}
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  {:else if error}
    <span>Error fetching the messages: {error}.</span>
  {:else}
    <div
      bind:this={scrollContainer}
      style="display: flex; flex-direction: column; overflow: auto; direction: ltr;"
    >
      {#each hashes as hash}
        <div style="margin-bottom: 8px;">
          <!-- svelte-ignore missing-declaration -->
          <Message messageHash={hash} on:room-joined={() => fetchMessages()}
          ></Message>
        </div>
      {/each}
    </div>
  {/if}
</div>
