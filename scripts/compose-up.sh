#!/bin/bash
set -e

ENV_FILE="./.env"
if [ ! -f "$ENV_FILE" ]; then
  echo ".env file not found!"
  exit 1
fi

# Export .env variables
set -a
source "$ENV_FILE"
set +a

echo "Starting Docker Compose..."
docker-compose up -d

echo "Waiting for PostgreSQL to be available in container '$CONTAINER_NAME'..." | sed 's/\r//g'
until docker exec -i "$CONTAINER_NAME" pg_isready -U "$SUPERUSER" > /dev/null 2>&1; do
  echo "PostgreSQL is unavailable - sleeping"
  sleep 2
done
echo "PostgreSQL is up!"

# Ensure the application user exists.
USER_EXISTS=$(docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -tAc "SELECT 1 FROM pg_roles WHERE rolname='$APP_USER';")
if [ "$USER_EXISTS" != "1" ]; then
  echo "Creating user '$APP_USER'..."
  docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -c "CREATE USER $APP_USER WITH ENCRYPTED PASSWORD '$APP_USER_PWD';"
else
  echo "User '$APP_USER' already exists."
fi

# Ensure that the application user has superuser privileges.
#SUPERUSER_CHECK=$(docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -tAc "SELECT rolsuper FROM pg_roles WHERE rolname='$APP_USER';")
#if [ "$SUPERUSER_CHECK" != "t" ]; then
#  echo "Granting superuser privileges to user '$APP_USER'..."
#  docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -c "ALTER USER $APP_USER WITH SUPERUSER;"
#else
#  echo "User '$APP_USER' already has superuser privileges."
#fi

# Ensure the application database exists.
DB_EXISTS=$(docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -tAc "SELECT 1 FROM pg_database WHERE datname='$APP_DB_NAME';")
if [ "$DB_EXISTS" != "1" ]; then
  echo "Creating database '$APP_DB_NAME'..."
  docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -c "CREATE DATABASE \"$APP_DB_NAME\";"
else
  echo "Database '$APP_DB_NAME' already exists."
fi

# Ensure the database owner is the application user.
DB_OWNER=$(docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -tAc "SELECT pg_catalog.pg_get_userbyid(datdba) FROM pg_database WHERE datname='$APP_DB_NAME';")
if [ "$DB_OWNER" != "$APP_USER" ]; then
  echo "Changing owner of '$APP_DB_NAME' to '$APP_USER'..."
  docker exec -i "$CONTAINER_NAME" psql -U "$SUPERUSER" -c "ALTER DATABASE \"$APP_DB_NAME\" OWNER TO $APP_USER;"
else
  echo "Database '$APP_DB_NAME' is already owned by '$APP_USER'."
fi

echo "Database initialization complete."

# Execute sqlx migrations.
export DATABASE_URL="postgres://$APP_USER:$APP_USER_PWD@localhost:$DB_PORT/$APP_DB_NAME"
echo "DATABASE_URL set to: $DATABASE_URL"
echo "Running sqlx migrations..."
sqlx migrate run
echo "Migrations complete."


echo "Docker Compose started successfully."
