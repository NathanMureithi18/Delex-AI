import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ChatMessage { 'content' : string, 'role' : string }
export interface _SERVICE {
  'chat' : ActorMethod<[Array<ChatMessage>], string>,
  'constitution_chat' : ActorMethod<[Array<ChatMessage>], string>,
  'constitution_prompt' : ActorMethod<[string], string>,
  'get_all_constitution_chunks' : ActorMethod<[], Array<string>>,
  'prompt' : ActorMethod<[string], string>,
  'search_constitution' : ActorMethod<[string], Array<string>>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
