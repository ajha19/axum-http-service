#!/bin/bash
set -e

# Start the server in the background
echo "Starting server..."
cargo run &
SERVER_PID=$!

# Wait for server to start (compilation might take time)
echo "Waiting for server to start..."
sleep 20

echo "---------------------------------------------------"
echo "1. Testing Health Endpoint (expecting JSON response)"
curl -v http://localhost:3000/health
echo ""
echo "---------------------------------------------------"

echo "2. Testing Request ID Middleware (check headers for x-request-id)"
curl -v http://localhost:3000/health 2>&1 | grep "x-request-id"
echo "---------------------------------------------------"

echo "3. Testing Rate Limiter (Allow burst 5, then block)"
echo "Sending 20 requests rapidly in parallel..."
# Fire 20 requests in parallel and count status codes
seq 1 20 | xargs -n1 -P20 curl -s -o /dev/null -w "%{http_code}\n" http://localhost:3000/health | sort | uniq -c
echo "---------------------------------------------------"

echo "4. Checking Metrics Endpoint"
curl -v http://localhost:3000/metrics
echo ""
echo "---------------------------------------------------"

echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null || true
echo "Demo complete."
