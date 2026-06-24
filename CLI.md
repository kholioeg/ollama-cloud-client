# CLI Testing Guide

## Quick Start

### 1. Setup Environment

```bash
# Navigate to project directory
cd ollama-cloud-client

# Copy and configure environment variables
cp .env.example .env

# Edit .env with your Ollama Cloud credentials
# OLLAMA_API_ENDPOINT=<your-endpoint>
# OLLAMA_API_KEY=<your-api-key>
```

### 2. Build the Project

```bash
source "$HOME/.cargo/env"
cargo build
```

### 3. Run the CLI

```bash
# Set up logging (optional)
RUST_LOG=info cargo run

# Or with debug logging for more details
RUST_LOG=debug cargo run
```

## CLI Commands

Once the app is running, you'll see a menu with the following interactions:

### Main Commands

| Command | Description |
|---------|-------------|
| `exit` | Quit the application |
| `history` | Show all past conversations |
| `models` | List available models |
| Regular text | Send a message to the current model |

### Workflow Example

```
1. App connects and lists available models
2. Select a model by number (e.g., "1" for first model)
3. Conversation starts automatically
4. Type messages to chat
5. Responses are saved automatically
6. Use "history" to see past conversations
7. Type "exit" to quit
```

## Example Session

```
╔══════════════════════════════════════════════════════╗
║     Ollama Cloud Client - CLI Interface (v0.1)      ║
╚══════════════════════════════════════════════════════╝

✓ Connected to Ollama Cloud!
Available models:
  1. llama2-7b
  2. mistral-7b
  3. neural-chat-7b

Select a model by number: 
1
Selected model: llama2-7b

✓ Started conversation (ID: abc-123-def-456)

Type 'exit' to quit, 'history' to see chat history

You: What is Rust?
Assistant: Rust is a systems programming language that emphasizes safety, speed, 
and concurrency. It's great for building reliable and efficient software...

You: history
╔════════════════════════════════════════════════════════╗
║      Conversation History                             ║
╚════════════════════════════════════════════════════════╝
ID: abc-123-def-456 | Title: CLI Chat

You: exit
✓ Goodbye!
```

## Features

✅ **API Integration**
- Connect to Ollama Cloud
- List available models
- Send chat messages
- Receive responses

✅ **Local Storage**
- Conversations saved to SQLite
- Message history persisted
- Automatic ID generation

✅ **Error Handling**
- Graceful connection failures
- Clear error messages
- Logging for debugging

## Troubleshooting

### Error: "Failed to connect to Ollama Cloud"

**Solution:**
1. Check `.env` file has correct values
2. Verify `OLLAMA_API_ENDPOINT` is accessible
3. Confirm `OLLAMA_API_KEY` is valid
4. Test with: `curl -H "Authorization: Bearer YOUR_KEY" http://your-endpoint/api/tags`

### Error: "No active conversation"

**Solution:**
1. Restart the app
2. Select a model from the list
3. Try sending a message again

### Database errors

**Solution:**
1. Check that `~/.ollama-cloud-client/` directory exists and is writable
2. Delete `conversations.db` and restart to reset the database
3. Increase log level: `RUST_LOG=debug cargo run`

## Logging Output

Logs are saved to the standard output. Control verbosity with `RUST_LOG`:

```bash
# Show only errors
RUST_LOG=error cargo run

# Show warnings and errors
RUST_LOG=warn cargo run

# Show info (recommended)
RUST_LOG=info cargo run

# Show debug info (detailed)
RUST_LOG=debug cargo run
```

## Next Steps

After testing the CLI:

1. **Verify API integration** - Test message sending and receiving
2. **Check database** - Inspect `~/.ollama-cloud-client/conversations.db` with sqlite3
3. **Review logs** - Look for any warnings or issues with `RUST_LOG=debug`
4. **Plan UI** - Use CLI feedback to inform UI/UX design

## Development Notes

The CLI uses:
- `ConversationManager` for business logic
- `OllamaClient` for API communication
- `Database` for persistence
- Standard input/output for user interaction

This can be replaced with a GUI later while keeping the backend logic intact.
