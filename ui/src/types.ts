import type {
  ActionHash,
  SignedActionHashed,
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink,
} from '@holochain/client'

export type Page = 'search' | 'rooms' | 'conversation'

export type ChatroomSignal =
  | {
      type: 'RemoteMessageCreated'
      action: SignedActionHashed<Create>
      app_entry: EntryTypes
      room_hash: ActionHash
    }
  | {
      type: 'EntryCreated'
      action: SignedActionHashed<Create>
      app_entry: EntryTypes
    }
  | {
      type: 'EntryUpdated'
      action: SignedActionHashed<Update>
      app_entry: EntryTypes
      original_app_entry: EntryTypes
    }
  | {
      type: 'EntryDeleted'
      action: SignedActionHashed<Delete>
      original_app_entry: EntryTypes
    }
  | {
      type: 'LinkCreated'
      action: SignedActionHashed<CreateLink>
      link_type: string
    }
  | {
      type: 'LinkDeleted'
      action: SignedActionHashed<DeleteLink>
      link_type: string
    }

export type EntryTypes =
  | ({ type: 'Message' } & Message)
  | ({ type: 'Room' } & Room)

export interface Room {
  name: string

  creator: AgentPubKey
}

export interface Message {
  content: string

  creator: AgentPubKey

  timestamp: number

  room_hash: ActionHash
}
