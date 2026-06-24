# ollama-cloud-client

A lightweight desktop client for Ollama Cloud that enables chat with supported cloud LLMs from a native Rust application.

## Overview

This project is a minimal viable product (MVP) desktop application written in Rust. It provides a straightforward interface for interacting with Ollama-supported language models hosted in the cloud.

## Features

- Native desktop client built with Rust
- Direct access to Ollama Cloud language models
- Simple chat-style interaction with cloud LLMs
- Focused on fast iteration and MVP delivery

## Getting Started

### Prerequisites

- Rust toolchain installed (`rustup`, `cargo`)
- Ollama Cloud credentials or access to the provider endpoint
- Desktop environment compatible with the chosen Rust GUI stack

### Run Locally

```bash
cargo run
```

> If the application requires configuration or API keys, set them in your environment or through your local configuration before running.

## Development

- `cargo build`: compile the application
- `cargo run`: launch the desktop client
- `cargo test`: run tests if available

## Project Goals

The goal of `ollama-cloud-client` is to provide an easy-to-use desktop interface for Ollama Cloud models. It is designed for:

- developers who want a local client for cloud LLMs
- users preferring a native desktop experience over browser-based apps
- rapid prototyping of chat workflows with cloud language models

## Future Improvements

Potential enhancements include:

- richer UI/UX for chat sessions
- model selection and configuration controls
- conversation logging and persistence
- support for more Ollama Cloud capabilities

## Notes

This repository is currently at an MVP stage. Contributions and feature proposals are welcome.
