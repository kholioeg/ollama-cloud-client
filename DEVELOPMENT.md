# Development Guide

## Project Overview

This is a Rust desktop client for Ollama Cloud LLMs with:
- **API Client**: REST communication with Ollama Cloud
- **Local Storage**: SQLite for conversation history
- **Async Runtime**: Tokio-based for high-performance I/O
- **Error Handling**: Comprehensive error types

## Getting Started

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source Cargo environment
source "$HOME/.cargo/env"
```

### Setup

1. **Clone and navigate to project**
   ```bash
   cd ollama-cloud-client
   ```

2. **Create `.env` file**
   ```bash
   cp .env.example .env
   # Edit .env with your Ollama Cloud credentials
   ```

3. **Build the project**
   ```bash
   cargo build
   ```

4. **Run**
   ```bash
   cargo run
   ```

## Project Structure

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library exports
├── api/              # Ollama Cloud API client
│   ├── mod.rs
│   ├── client.rs     # HTTP client implementation
│   └── models.rs     # Data models (Message, ChatRequest, etc.)
├── storage/          # Local database
│   ├── mod.rs
│   └── database.rs   # SQLite operations
├── config.rs         # Configuration from environment
└── error.rs          # Error types

Cargo.toml           # Project manifest & dependencies
.env.example         # Environment variables template
.gitignore          # Git ignore rules
```

## Key Components

### API Client (`src/api/client.rs`)

```rust
// List available models
let client = OllamaClient::new(endpoint, api_key)?;
let models = client.list_models().await?;

// Send a chat message
let response = client.chat("model-name", messages).await?;

// Test connectivity
let is_healthy = client.health_check().await?;
```

### Storage (`src/storage/database.rs`)

```rust
// Initialize database
let db = Database::new("~/.ollama-cloud-client/conversations.db").await?;

// Create a conversation
let conv_id = db.create_conversation("My Chat", "model-name").await?;

// Save a message
db.save_message(&conv_id, &message).await?;

// List conversations
let conversations = db.list_conversations().await?;
```

### Configuration (`src/config.rs`)

Environment variables (from `.env`):
- `OLLAMA_API_ENDPOINT`: API server URL (default: `http://localhost:11434`)
- `OLLAMA_API_KEY`: Authentication token
- `DB_PATH`: SQLite database location
- `LOG_LEVEL`: Log verbosity (debug, info, warn, error)

## Development Commands

```bash
# Check for compile errors without building
cargo check

# Build the project
cargo build

# Build in release mode (optimized)
cargo build --release

# Run the application
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Generate documentation
cargo doc --open
```

## Testing

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_client_creation

# Run tests with output
cargo test -- --nocapture
```

## Next Development Steps

### Phase 1: Core Chat Loop
- [ ] Implement CLI interface for testing
- [ ] Create conversation manager
- [ ] Build message handler (send → receive → store)
- [ ] Add streaming response collection

### Phase 2: UI Layer
- [ ] Integrate Tauri (or chosen GUI framework)
- [ ] Design chat window layout
- [ ] Implement model selector
- [ ] Add settings panel

### Phase 3: Features
- [ ] Conversation history browsing
- [ ] Markdown rendering in responses
- [ ] Export conversations
- [ ] Syntax highlighting for code blocks

## Dependencies Overview

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `sqlx` | Database ORM |
| `serde` | Serialization |
| `log` | Logging framework |
| `chrono` | Timestamps |
| `uuid` | Unique IDs |
| `dotenv` | Environment loading |

## Common Issues

### "Command not found: cargo"
```bash
source "$HOME/.cargo/env"
```

### Database connection errors
Ensure the directory for `DB_PATH` exists:
```bash
mkdir -p ~/.ollama-cloud-client
```

### API connection fails
Verify `OLLAMA_API_ENDPOINT` and `OLLAMA_API_KEY` in `.env`:
```bash
curl -H "Authorization: Bearer YOUR_KEY" http://your-endpoint/api/tags
```

## Code Style

This project follows Rust conventions:
- Run `cargo fmt` before committing
- Use `cargo clippy` to catch common mistakes
- Write tests for new functionality
- Document public APIs with doc comments

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [SQLx Guide](https://github.com/launchbadge/sqlx)
- [Tauri Docs](https://tauri.app/docs/) (for UI)
