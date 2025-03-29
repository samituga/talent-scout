#!/bin/bash

set -e

DB_CONTAINER_NAME="talent-scout-db"
POSTGRES_DATA_PATH="/data/postgres"
COMPOSE_SCRIPT="./scripts/compose-up.sh"

echo "Stopping Docker container '$DB_CONTAINER_NAME'..."
docker stop "$DB_CONTAINER_NAME" || true

echo "Removing Docker container '$DB_CONTAINER_NAME'..."
docker rm "$DB_CONTAINER_NAME" || true

echo "Removing PostgreSQL data directory at '$POSTGRES_DATA_PATH'..."
sudo rm -rf "$POSTGRES_DATA_PATH"

echo "Starting Docker Compose..."
bash "$COMPOSE_SCRIPT"
