<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import { decode } from '@msgpack/msgpack'
  import type {
    Record,
    ActionHash,
    AppClient,
    EntryHash,
    AgentPubKey,
    DnaHash,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Room } from './types'
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

  $: error, loading, record, room

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

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <span>Error fetching the room: {error}</span>
{:else}
  <div style="display: flex; flex-direction: column">
    <div style="display: flex; flex-direction: row">
      <span style="flex: 1"></span>
    </div>
  </div>
{/if}
