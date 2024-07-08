<script lang="ts">
  import Banner from './Banner.svelte'
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

<Banner challengeName={'Scaffolding & Signals'} challengeNumber={4}>
  <div style="display: flex; flex-direction: column-reverse; height: 100%;">
    <NavBar setpage={(p) => (page = p)} {page} />
    <div
      style="height: calc(100% - 64px); display: flex; flex-direction: column; "
    >
      {#if loading}
        <mwc-circular-progress indeterminate />
      {:else if page === 'search'}
        <div
          style="max-width: 600px; margin: 0 auto; width: 100%; display: flex; justify-content: center; flex-direction: column; height: 100%;"
        >
          <SearchRooms
            on:room-created={(e) => openChatRoom(e.detail.roomHash)}
            on:room-joined={(e) => openChatRoom(e.detail.roomHash)}
          />
        </div>
      {:else if page === 'rooms'}
        <div
          style="max-width: 600px; margin: 0 auto; width: 100%; display: flex; justify-content: center; flex-direction: column; height: 100%;"
        >
          <YourRooms {openChatRoom} />
        </div>
      {:else if page === 'conversation'}
        <Conversation roomHash={chatRoomHash} />
      {/if}
    </div>
  </div>
</Banner>
