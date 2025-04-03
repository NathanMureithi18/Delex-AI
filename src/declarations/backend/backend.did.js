export const idlFactory = ({ IDL }) => {
  const ChatMessage = IDL.Record({ 'content' : IDL.Text, 'role' : IDL.Text });
  return IDL.Service({
    'chat' : IDL.Func([IDL.Vec(ChatMessage)], [IDL.Text], []),
    'constitution_chat' : IDL.Func([IDL.Vec(ChatMessage)], [IDL.Text], []),
    'constitution_prompt' : IDL.Func([IDL.Text], [IDL.Text], []),
    'get_all_constitution_chunks' : IDL.Func(
        [],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'prompt' : IDL.Func([IDL.Text], [IDL.Text], []),
    'search_constitution' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
