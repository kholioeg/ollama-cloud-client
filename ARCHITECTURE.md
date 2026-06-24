# Architecture

## System Design

```
┌─────────────────────────────────────────────────────────┐
│                    User Interface Layer                  │
│              (Tauri / Desktop GUI - Future)              │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│                  Application Logic Layer                 │
│          - Message handling & routing                    │
│          - Conversation management                       │
│          - State management                              │
└─────────────────────────────────────────────────────────┘
                           ↓
        ┌──────────────────┬──────────────────┐
        ↓                  ↓                  ↓
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│  API Layer   │   │ Storage Layer│   │Config Layer  │
│              │   │              │   │              │
│• OllamaClient│   │• Database    │   │• Environment │
│• HTTP Client │   │• SQLite      │   │• Settings    │
│• Models      │   │• Conversations
│              │   │• Messages    │   │              │
└──────────────┘   └──────────────┘   └──────────────┘
        ↓                  ↓
   Ollama Cloud API   Local SQLite DB
