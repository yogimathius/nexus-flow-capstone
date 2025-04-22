#!/bin/bash

# Default port if not specified
PORT=${1:-3003}

echo "Testing API on port $PORT..."

# Test health endpoint
echo "Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:$PORT/health)
if [ "$HEALTH_RESPONSE" == "200" ]; then
    echo "✅ Health check passed!"
else
    echo "❌ Health check failed! Status: $HEALTH_RESPONSE"
    echo "Make sure your server is running on port $PORT"
fi

# Test dream interpretation endpoint
echo -e "\nTesting dream interpretation endpoint..."
DREAM_RESPONSE=$(curl -s -X POST http://localhost:$PORT/api/interpret_dream \
  -H "Content-Type: application/json" \
  -d '{"dream_text": "I was flying over a city made of crystal."}')

if [ -z "$DREAM_RESPONSE" ]; then
    echo "❌ No response received from dream interpretation endpoint!"
    echo "Make sure your server is running on port $PORT"
else
    echo "Response received:"
    echo $DREAM_RESPONSE | python3 -m json.tool || echo $DREAM_RESPONSE
fi 