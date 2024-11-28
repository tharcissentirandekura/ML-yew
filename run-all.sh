#!/bin/bash

# Run the API in the background
echo "Starting the API..."
cargo run --bin api &
API_PID=$!

# Wait a moment for the API to start
sleep 2
echo "API running on http://127.0.0.1:8000"

# Run the frontend in the background
echo "Starting the frontend..."
cd frontend
trunk serve &
FRONTEND_PID=$!

echo "Frontend running on http://127.0.0.1:8080"

# Handle script termination
trap "kill $API_PID $FRONTEND_PID" EXIT

# Wait for both processes
wait $API_PID $FRONTEND_PID