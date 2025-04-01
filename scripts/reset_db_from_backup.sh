#!/bin/bash
set -e

DB_CONTAINER_NAME="talent-scout-db"
DB_VOLUME_PATH="/data/postgres/talent-scout"
BACKUP_PATH="/data/postgres/talent-scout-backup"
COMPOSE_SCRIPT="./scripts/compose-up.sh"

echo "Stopping Docker container '$DB_CONTAINER_NAME'..."
docker stop "$DB_CONTAINER_NAME" || true

echo "Removing Docker container '$DB_CONTAINER_NAME'..."
docker rm "$DB_CONTAINER_NAME" || true

echo "Removing PostgreSQL volume directory at '$DB_VOLUME_PATH'..."
sudo rm -rf "$DB_VOLUME_PATH"

echo "Restoring backup from '$BACKUP_PATH' to '$DB_VOLUME_PATH'..."
sudo cp -a "$BACKUP_PATH" "$DB_VOLUME_PATH"

echo "Starting Docker Compose..."
bash "$COMPOSE_SCRIPT"
