#!/bin/bash

echo "Starting Legal Research System..."

# Check if .env file exists
if [ ! -f .env ]; then
    echo "Creating .env file from template..."
    cp .env.example .env
    echo "Please edit .env file with your database URL and other settings"
    echo "Minimum required: DATABASE_URL"
    exit 1
fi

# Load environment variables
source .env

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: DATABASE_URL not set in .env file"
    echo "Please set DATABASE_URL in .env file"
    exit 1
fi

echo "Building application..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed. Please fix errors and try again."
    exit 1
fi

echo "Starting server..."
echo "Access the application at: http://localhost:${PORT:-8080}"
echo "Press Ctrl+C to stop the server"

cargo run --release