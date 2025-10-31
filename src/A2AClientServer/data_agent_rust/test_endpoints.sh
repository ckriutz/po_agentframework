#!/bin/bash

echo "🔍 A2A Agent Card Endpoint Testing"
echo "================================="

BASE_URL="http://localhost:8080"

# Common A2A agent card endpoint patterns
ENDPOINTS=(
    "/.well-known/agent.json"
    "/agent.json"
    "/a2a/agent.json"
    "/agent/card"
    "/agent/card.json"
    "/.well-known/a2a.json"
    "/api/agent.json"
    "/api/agent/card"
    "/v1/agent.json"
    "/v1/agent/card"
)

echo "Testing common A2A agent card endpoints:"
echo ""

for endpoint in "${ENDPOINTS[@]}"; do
    url="${BASE_URL}${endpoint}"
    
    response=$(curl -s -w "%{http_code}" -o /dev/null "$url")
    
    if [ "$response" = "200" ]; then
        echo "✅ $endpoint - Working (HTTP 200)"
    else
        echo "❌ $endpoint - Failed (HTTP $response)"
    fi
done

echo ""
echo "📝 Test with your C# client using these base URLs:"
echo "   Base URL: $BASE_URL"
echo "   Alternative: $BASE_URL/api"
echo "   Alternative: $BASE_URL/a2a"
echo ""
echo "💡 If your C# client is using a specific path like '/agent'"
echo "   then try: http://localhost:8080/agent (as base URL)"
echo "   but make sure the final constructed URL matches one of our working endpoints"