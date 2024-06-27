<script lang="ts">
  import { onMount, setContext } from 'svelte'
  import type { ActionHash, AppClient } from '@holochain/client'
  import { AppWebsocket } from '@holochain/client'
  import '@material/mwc-circular-progress'

  import { clientContext } from './contexts'

  import SearchRooms from './chatroom/premade-chatroom-ui/SearchRooms.svelte'
  import Conversation from './chatroom/premade-chatroom-ui/Conversation.svelte'

  import type { Page } from './types'
  import NavBar from './NavBar.svelte'
  import YourRooms from './chatroom/premade-chatroom-ui/YourRooms.svelte'

  let client: AppClient | undefined

  let loading = true

  let page: Page = 'search'
  let chatRoomHash: ActionHash = undefined

  $: page, chatRoomHash

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppWebsocket.connect()

    loading = false
  })

  setContext(clientContext, {
    getClient: () => client,
  })

  function openChatRoom(roomHash: ActionHash) {
    page = 'conversation'
    chatRoomHash = roomHash
  }
</script>

<main style="text-align: center;">
  <div style="display: flex; flex-direction: column-reverse; height: 100vh;">
    <NavBar setpage={(p) => (page = p)} {page} />
    <div
      style="display: flex; justify-content: center; align-items: center; height: 100%;"
    >
      {#if loading}
        <div
          style="display: flex; flex: 1; align-items: center; justify-content: center;"
        >
          <mwc-circular-progress indeterminate />
        </div>
      {:else}
        <div
          id="content"
          style="max-width: 400px; margin-left: auto; margin-right: auto; display: flex; flex-direction: column; flex: 1; justify-content: center; gap: 20px;"
        >
          {#if page === 'search'}
            <SearchRooms
              on:room-created={(e) => openChatRoom(e.detail.roomHash)}
              on:room-joined={(e) => openChatRoom(e.detail.roomHash)}
            />
          {:else if page === 'rooms'}
            <YourRooms {openChatRoom} />
          {:else if page === 'conversation'}
            <Conversation roomHash={chatRoomHash} />
          {/if}
        </div>
      {/if}
    </div>
  </div>
</main>
