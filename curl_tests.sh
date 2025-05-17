#!/bin/bash

# Set the API base URL
API_URL="http://localhost:8080"

# Text colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print header
print_header() {
  echo -e "\n${BLUE}=== $1 ===${NC}\n"
}

# Function to check if response was successful
check_response() {
  if [[ $1 == *"\"status\":\"success\""* ]]; then
    echo -e "${GREEN}✓ Success${NC}"
  else
    echo -e "${RED}✗ Failed${NC}"
    echo -e "${RED}Response: $1${NC}"
  fi
}

# Function to check if validation error response was as expected
check_validation_error() {
  if [[ $1 == *"\"status\":\"error\""* && $1 == *"\"errors\""* ]]; then
    echo -e "${GREEN}✓ Expected validation error returned${NC}"
  else
    echo -e "${RED}✗ Validation error response was not as expected${NC}"
    echo -e "${RED}Response: $1${NC}"
  fi
}

# Test health endpoint
print_header "Testing Health Endpoint"
HEALTH_RESPONSE=$(curl -s "$API_URL/health")
echo "$HEALTH_RESPONSE" | jq .
check_response "$HEALTH_RESPONSE"

# Test dream interpretation endpoint
print_header "Testing Dream Interpretation"
INTERPRETATION_RESPONSE=$(curl -s -X POST \
  -H "Content-Type: application/json" \
  -d '{"dream_text":"I was flying over mountains of gold"}' \
  "$API_URL/api/interpret_dream")
echo "$INTERPRETATION_RESPONSE" | jq .
check_response "$INTERPRETATION_RESPONSE"

# Test dream interpretation validation
print_header "Testing Dream Interpretation Validation"
VALIDATION_RESPONSE=$(curl -s -X POST \
  -H "Content-Type: application/json" \
  -d '{"dream_text":""}' \
  "$API_URL/api/interpret_dream")
echo "$VALIDATION_RESPONSE" | jq .
check_validation_error "$VALIDATION_RESPONSE"

# Test create dream endpoint
print_header "Testing Dream Creation"
CREATE_RESPONSE=$(curl -s -X POST \
  -H "Content-Type: application/json" \
  -d '{"text":"I was swimming in a luminous ocean", "interpretation":"This dream symbolizes emotional clarity"}' \
  "$API_URL/api/dreams")
echo "$CREATE_RESPONSE" | jq .
check_response "$CREATE_RESPONSE"

# Extract the ID for later use
DREAM_ID=$(echo "$CREATE_RESPONSE" | jq -r '.id')

# Test create dream validation
print_header "Testing Dream Creation Validation"
CREATE_VALIDATION_RESPONSE=$(curl -s -X POST \
  -H "Content-Type: application/json" \
  -d '{"text":"", "interpretation":"This is not valid"}' \
  "$API_URL/api/dreams")
echo "$CREATE_VALIDATION_RESPONSE" | jq .
check_validation_error "$CREATE_VALIDATION_RESPONSE"

# Test list dreams endpoint
print_header "Testing Dreams Listing"
LIST_RESPONSE=$(curl -s "$API_URL/api/dreams")
echo "$LIST_RESPONSE" | jq .
# Check if it's an array
if [[ $(echo "$LIST_RESPONSE" | jq 'if type=="array" then "array" else "not_array" end') == "\"array\"" ]]; then
  echo -e "${GREEN}✓ Response is an array${NC}"
else
  echo -e "${RED}✗ Response is not an array${NC}"
  echo -e "${RED}Response: $LIST_RESPONSE${NC}"
fi

# Test get dream by ID endpoint
print_header "Testing Get Dream by ID"
GET_RESPONSE=$(curl -s "$API_URL/api/dreams/$DREAM_ID")
echo "$GET_RESPONSE" | jq .
check_response "$GET_RESPONSE"

# Test get non-existent dream
print_header "Testing Get Non-existent Dream"
random_uuid=$(uuidgen)
NOT_FOUND_RESPONSE=$(curl -s "$API_URL/api/dreams/$random_uuid")
echo "$NOT_FOUND_RESPONSE" | jq .
if [[ $NOT_FOUND_RESPONSE == *"\"status\":\"error\""* && $NOT_FOUND_RESPONSE == *"\"code\":\"NOT_FOUND\""* ]]; then
  echo -e "${GREEN}✓ Expected not found error returned${NC}"
else
  echo -e "${RED}✗ Not found error response was not as expected${NC}"
  echo -e "${RED}Response: $NOT_FOUND_RESPONSE${NC}"
fi

echo -e "\n${GREEN}=== All tests completed ===${NC}\n" 