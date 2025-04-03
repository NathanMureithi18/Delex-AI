import React, { useState, useRef, useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { backend } from 'declarations/backend';
import botImg from '/bot.svg';
import userImg from '/user.svg';
import '/index.css';

const App = () => {
  const [chat, setChat] = useState([
    {
      role: "system",
      content: "I'm a legal assistant specialized in Kenyan constitutional law. Ask me anything about the Kenyan Constitution."
    }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const chatBoxRef = useRef(null);

  const formatDate = (date) => {
    const h = '0' + date.getHours();
    const m = '0' + date.getMinutes();
    return `${h.slice(-2)}:${m.slice(-2)}`;
  };

  const askAgent = async (messages) => {
    try {
      // Convert messages to the format expected by your backend
      const formattedMessages = messages.map(msg => ({
        role: msg.role,
        content: msg.content
      }));

      // Use constitution_chat instead of chat
      const response = await backend.constitution_chat(formattedMessages);
      
      setChat((prevChat) => {
        const newChat = [...prevChat];
        newChat.pop(); // Remove the "Thinking..." message
        newChat.push({ role: "system", content: response });
        return newChat;
      });
    } catch (e) {
      console.log(e);
      const eStr = String(e);
      const match = eStr.match(/(SysTransient|CanisterReject), \\+"([^\\"]+)/);
      if (match) {
        alert(match[2]);
      } else {
        alert("An error occurred while communicating with the canister");
      }
      
      setChat((prevChat) => {
        const newChat = [...prevChat];
        newChat.pop(); // Remove the "Thinking..." message
        return newChat;
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!inputValue.trim() || isLoading) return;

    const userMessage = {
      role: "user",
      content: inputValue
    };
    
    const thinkingMessage = {
      role: "system",
      content: 'Thinking ...'
    };
    
    setChat((prevChat) => [...prevChat, userMessage, thinkingMessage]);
    setInputValue('');
    setIsLoading(true);
    
    // Send all messages for context
    const messagesToSend = chat.concat(userMessage);
    askAgent(messagesToSend);
  };

  // Add a function to search the constitution directly
  const searchConstitution = async (keyword) => {
    try {
      setIsLoading(true);
      const results = await backend.search_constitution(keyword);
      
      if (results.length === 0) {
        setChat(prev => [...prev, {
          role: "system",
          content: `No results found for "${keyword}" in the Kenyan Constitution.`
        }]);
      } else {
        const formattedResults = results.join("\n\n");
        setChat(prev => [...prev, {
          role: "system",
          content: `Search results for "${keyword}":\n\n${formattedResults}`
        }]);
      }
    } catch (e) {
      console.error(e);
      alert("Error searching the constitution");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    if (chatBoxRef.current) {
      chatBoxRef.current.scrollTop = chatBoxRef.current.scrollHeight;
    }
  }, [chat]);

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 p-4">
      <div className="flex h-[80vh] w-full max-w-2xl flex-col rounded-lg bg-white shadow-lg">
        <div className="bg-blue-600 text-white p-4 rounded-t-lg">
          <h1 className="text-xl font-bold">DeLex AI</h1>
        </div>
        
        <div className="flex-1 overflow-y-auto bg-gray-100 p-4" ref={chatBoxRef}>
          {chat.map((message, index) => {
            const isUser = message.role === "user";
            const img = isUser ? userImg : botImg;
            const name = isUser ? 'User' : 'Legal Assistant';
            const text = message.content;

            return (
              <div key={index} className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-4`}>
                {!isUser && (
                  <div
                    className="mr-2 h-10 w-10 rounded-full"
                    style={{ backgroundImage: `url(${img})`, backgroundSize: 'cover' }}
                  ></div>
                )}
                <div className={`max-w-[70%] rounded-lg p-3 ${isUser ? 'bg-blue-500 text-white' : 'bg-white shadow'}`}>
                  <div
                    className={`mb-1 flex items-center justify-between text-sm ${isUser ? 'text-white' : 'text-gray-500'}`}
                  >
                    <div>{name}</div>
                    <div className="mx-2">{formatDate(new Date())}</div>
                  </div>
                  <div className="whitespace-pre-wrap">{text}</div>
                </div>
                {isUser && (
                  <div
                    className="ml-2 h-10 w-10 rounded-full"
                    style={{ backgroundImage: `url(${img})`, backgroundSize: 'cover' }}
                  ></div>
                )}
              </div>
            );
          })}
        </div>
        
        <div className="bg-gray-200 p-2">
          <div className="flex space-x-2 mb-2">
            <button 
              onClick={() => setInputValue("What are the fundamental rights in the Kenyan Constitution?")}
              className="bg-gray-300 hover:bg-gray-400 text-sm px-2 py-1 rounded"
              disabled={isLoading}
            >
              Fundamental Rights
            </button>
            <button 
              onClick={() => setInputValue("Explain citizenship by birth in Kenya")}
              className="bg-gray-300 hover:bg-gray-400 text-sm px-2 py-1 rounded"
              disabled={isLoading}
            >
              Citizenship
            </button>
            <button 
              onClick={() => setInputValue("What does the constitution say about devolution?")}
              className="bg-gray-300 hover:bg-gray-400 text-sm px-2 py-1 rounded"
              disabled={isLoading}
            >
              Devolution
            </button>
          </div>
        </div>
        
        <form className="flex rounded-b-lg border-t bg-white p-4" onSubmit={handleSubmit}>
          <input
            type="text"
            className="flex-1 rounded-l border p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Ask about the Kenyan Constitution..."
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            disabled={isLoading}
          />
          <button
            type="submit"
            className="rounded-r bg-blue-500 p-2 text-white hover:bg-blue-600 disabled:bg-blue-300"
            disabled={isLoading}
          >
            Send
          </button>
        </form>
      </div>
    </div>
  );
};

export default App;

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);