#!/bin/bash

# Default port if not provided
PORT=${1:-3003}

# Check if we have the necessary environment variables
if [ -z "$OPENROUTER_API_KEY" ]; then
    echo "OPENROUTER_API_KEY is not set. Please set it in your .env file or export it."
    echo "Example: export OPENROUTER_API_KEY=your_key_here"
    exit 1
fi

echo "Starting server on port $PORT with OpenRouter API key"

# Check for GitHub token for Azure API
if [ -n "$GITHUB_TOKEN" ]; then
    echo "GitHub token found, Azure API will be available"
else
    echo "GITHUB_TOKEN not found. Only OpenRouter API will be available."
    echo "To use Azure API, set GITHUB_TOKEN in your environment."
    echo "Example: export GITHUB_TOKEN=your_github_token"
fi

cargo run -- --port $PORT 