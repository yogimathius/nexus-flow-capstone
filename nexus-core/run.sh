#!/bin/bash

# Default port if not provided
PORT=${1:-3003}

echo "Starting Nexus Core on port $PORT"
if [ -z "$OPENROUTER_API_KEY" ] && [ -z "$GITHUB_TOKEN" ]; then
    echo "No AI provider configured; interpretation requests will return 503."
fi

cargo run -- --port "$PORT"
