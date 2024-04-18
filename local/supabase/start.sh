#!/bin/bash

# https://supabase.com/docs/guides/self-hosting/docker

cd "$(dirname "$0")"

cd supabase_repo_clone/docker

# Copy the fake env vars
cp .env.example .env

# Pull the latest images
docker compose pull

# Start the services (in detached mode)
docker compose up -d