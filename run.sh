#!/bin/bash

# Quick start script for Ollama Cloud Client

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Ollama Cloud Client - Quick Start${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}\n"

# Check if .env exists
if [ ! -f .env ]; then
    echo -e "${YELLOW}⚠ .env file not found${NC}"
    echo "Creating .env from template..."
    cp .env.example .env
    echo -e "${YELLOW}Please edit .env with your Ollama Cloud credentials${NC}"
    echo "Edit the following in .env:"
    echo "  - OLLAMA_API_ENDPOINT=<your-endpoint>"
    echo "  - OLLAMA_API_KEY=<your-api-key>"
    exit 1
fi

# Source Cargo environment
source "$HOME/.cargo/env" 2>/dev/null || true

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠ Cargo not found. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo -e "${GREEN}✓ Rust toolchain ready${NC}\n"

# Build if needed
if [ ! -f target/debug/ollama-cloud-client ]; then
    echo -e "${BLUE}Building project...${NC}"
    cargo build
    echo ""
fi

# Run the application
echo -e "${BLUE}Starting Ollama Cloud Client...${NC}\n"

# Set default log level if not specified
export RUST_LOG="${RUST_LOG:-info}"

cargo run

