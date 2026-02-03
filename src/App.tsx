import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import EmojiPicker, { EmojiClickData } from 'emoji-picker-react';
import './App.css';

interface Peer {
  id: string;
  name: string;
  ip: string;
  port: number;
  last_seen: number;
}

interface Message {
  id: string;
  from_id: string;
  from_name: string;
  to_id: string;
  content: string;
  timestamp: number;
  is_file: boolean;
  file_name?: string;
  file_data?: string;
}

function App() {
  const [myId, setMyId] = useState('');
  const [myName, setMyName] = useState('');
  const [peers, setPeers] = useState<Peer[]>([]);
  const [selectedPeer, setSelectedPeer] = useState<Peer | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [showEmojiPicker, setShowEmojiPicker] = useState(false);
  const [sending, setSending] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Get my info
    invoke<[string, string]>('get_my_info').then(([id, name]) => {
      setMyId(id);
      setMyName(name);
    });

    // Request notification permission
    isPermissionGranted().then(granted => {
      if (!granted) {
        requestPermission();
      }
    });

    // Poll for peers
    const peerInterval = setInterval(() => {
      invoke<Peer[]>('get_peers').then(setPeers);
    }, 2000);

    // Listen for incoming messages
    const unlisten = listen<Message>('message_received', (event) => {
      setMessages(prev => [...prev, event.payload]);

      // Show notification if window not focused
      if (document.hidden) {
        sendNotification({
          title: `Message from ${event.payload.from_name}`,
          body: event.payload.is_file
            ? `Sent a file: ${event.payload.file_name}`
            : event.payload.content.substring(0, 50)
        });
      }
    });

    return () => {
      clearInterval(peerInterval);
      unlisten.then(fn => fn());
    };
  }, []);

  useEffect(() => {
    if (selectedPeer) {
      invoke<Message[]>('get_messages', { peerId: selectedPeer.id }).then(setMessages);
    }
  }, [selectedPeer]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSendMessage = async () => {
    if (!inputMessage.trim() || !selectedPeer || sending) return;

    setSending(true);
    try {
      const msg = await invoke<Message>('send_message', {
        toId: selectedPeer.id,
        content: inputMessage
      });
      setMessages(prev => [...prev, msg]);
      setInputMessage('');
    } catch (e) {
      console.error('Failed to send message:', e);
    }
    setSending(false);
  };

  const handleSendFile = async () => {
    if (!selectedPeer || sending) return;

    const file = await open({
      multiple: false,
      directory: false
    });

    if (file) {
      setSending(true);
      try {
        const msg = await invoke<Message>('send_file', {
          toId: selectedPeer.id,
          filePath: file
        });
        setMessages(prev => [...prev, msg]);
      } catch (e) {
        console.error('Failed to send file:', e);
      }
      setSending(false);
    }
  };

  const handleDownloadFile = (msg: Message) => {
    if (!msg.file_data || !msg.file_name) return;

    const binary = atob(msg.file_data);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) {
      bytes[i] = binary.charCodeAt(i);
    }

    const blob = new Blob([bytes]);
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = msg.file_name;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleEmojiClick = (emojiData: EmojiClickData) => {
    setInputMessage(prev => prev + emojiData.emoji);
    setShowEmojiPicker(false);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  return (
    <div className="app">
      {/* Header */}
      <header className="header">
        <div className="logo">
          <span className="logo-icon"></span>
          <span className="logo-text">LAN Chat</span>
        </div>
        <div className="user-info">
          <span className="user-name">{myName}</span>
          <span className="user-status">Online</span>
        </div>
      </header>

      <div className="main-content">
        {/* Sidebar - Peer List */}
        <aside className="sidebar">
          <div className="sidebar-header">
            <h2>Contacts</h2>
            <span className="peer-count">{peers.length} online</span>
          </div>
          <div className="peer-list">
            {peers.length === 0 ? (
              <div className="no-peers">
                <div className="no-peers-icon"></div>
                <p>Looking for peers...</p>
                <span>Make sure other devices are running LAN Chat on the same network</span>
              </div>
            ) : (
              peers.map(peer => (
                <div
                  key={peer.id}
                  className={`peer-item ${selectedPeer?.id === peer.id ? 'selected' : ''}`}
                  onClick={() => setSelectedPeer(peer)}
                >
                  <div className="peer-avatar">
                    {peer.name.charAt(0).toUpperCase()}
                  </div>
                  <div className="peer-details">
                    <span className="peer-name">{peer.name}</span>
                    <span className="peer-ip">{peer.ip}</span>
                  </div>
                  <div className="peer-status-dot"></div>
                </div>
              ))
            )}
          </div>
        </aside>

        {/* Chat Area */}
        <main className="chat-area">
          {selectedPeer ? (
            <>
              {/* Chat Header */}
              <div className="chat-header">
                <div className="chat-peer-info">
                  <div className="chat-peer-avatar">
                    {selectedPeer.name.charAt(0).toUpperCase()}
                  </div>
                  <div className="chat-peer-details">
                    <span className="chat-peer-name">{selectedPeer.name}</span>
                    <span className="chat-peer-ip">{selectedPeer.ip}</span>
                  </div>
                </div>
              </div>

              {/* Messages */}
              <div className="messages-container">
                {messages.length === 0 ? (
                  <div className="no-messages">
                    <p>No messages yet</p>
                    <span>Send a message to start the conversation</span>
                  </div>
                ) : (
                  messages.map(msg => (
                    <div
                      key={msg.id}
                      className={`message ${msg.from_id === myId ? 'sent' : 'received'}`}
                    >
                      {msg.is_file ? (
                        <div className="file-message" onClick={() => handleDownloadFile(msg)}>
                          <div className="file-icon"></div>
                          <div className="file-info">
                            <span className="file-name">{msg.file_name}</span>
                            <span className="file-action">Click to download</span>
                          </div>
                        </div>
                      ) : (
                        <p className="message-content">{msg.content}</p>
                      )}
                      <span className="message-time">{formatTime(msg.timestamp)}</span>
                    </div>
                  ))
                )}
                <div ref={messagesEndRef} />
              </div>

              {/* Input Area */}
              <div className="input-area">
                <div className="input-container">
                  <button
                    className="emoji-button"
                    onClick={() => setShowEmojiPicker(!showEmojiPicker)}
                  >
                    <svg viewBox="0 0 24 24" width="24" height="24">
                      <circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" strokeWidth="2"/>
                      <circle cx="9" cy="10" r="1.5" fill="currentColor"/>
                      <circle cx="15" cy="10" r="1.5" fill="currentColor"/>
                      <path d="M8 14s1.5 2 4 2 4-2 4-2" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                    </svg>
                  </button>

                  <textarea
                    className="message-input"
                    placeholder="Type a message..."
                    value={inputMessage}
                    onChange={(e) => setInputMessage(e.target.value)}
                    onKeyPress={handleKeyPress}
                    rows={1}
                  />

                  <button className="attach-button" onClick={handleSendFile} disabled={sending}>
                    <svg viewBox="0 0 24 24" width="24" height="24">
                      <path d="M21.44 11.05l-9.19 9.19a6 6 0 01-8.49-8.49l9.19-9.19a4 4 0 015.66 5.66l-9.2 9.19a2 2 0 01-2.83-2.83l8.49-8.48" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                    </svg>
                  </button>

                  <button
                    className="send-button"
                    onClick={handleSendMessage}
                    disabled={!inputMessage.trim() || sending}
                  >
                    <svg viewBox="0 0 24 24" width="24" height="24">
                      <path d="M22 2L11 13M22 2L15 22L11 13L2 9L22 2Z" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                    </svg>
                  </button>
                </div>

                {showEmojiPicker && (
                  <div className="emoji-picker-container">
                    <EmojiPicker onEmojiClick={handleEmojiClick} />
                  </div>
                )}
              </div>
            </>
          ) : (
            <div className="no-chat-selected">
              <div className="welcome-graphic">
                <div className="geometric-shape shape-1"></div>
                <div className="geometric-shape shape-2"></div>
                <div className="geometric-shape shape-3"></div>
                <div className="geometric-shape shape-4"></div>
              </div>
              <h2>Welcome to LAN Chat</h2>
              <p>Select a contact to start messaging</p>
            </div>
          )}
        </main>
      </div>
    </div>
  );
}

export default App;
