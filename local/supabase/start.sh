#!/bin/bash

SUPABASE_TAG=v1.24.09

cd "$(dirname "${BASH_SOURCE[0]}")" || exit

rm -rf supabase

# Get the code using git sparse checkout
git clone --filter=blob:none --branch "$SUPABASE_TAG" --no-checkout https://github.com/supabase/supabase
cd supabase || exit
git sparse-checkout set --cone docker && git checkout master

# Go to the docker folder
cd docker || exit

# Copy the env vars from guessfreak local folder
cp ../../.env .

# Pull the latest images
docker compose pull

# Start the services (in detached mode)
docker compose -f docker-compose.yml -f docker-compose.s3.yml up -d
