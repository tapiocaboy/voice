#!/bin/bash

# Export environment variables
export DATABASE_URL=postgres://user:password@localhost:5432/voice_analytics
export REDIS_URL=redis://localhost:6379
export PORT=8000
export JWT_SECRET=your-secret-key-here
export RUST_LOG=debug

# Print confirmation
echo "Environment variables set:"
echo "DATABASE_URL: $DATABASE_URL"
echo "REDIS_URL: $REDIS_URL"
echo "PORT: $PORT"
echo "JWT_SECRET: $JWT_SECRET"
echo "RUST_LOG: $RUST_LOG" 