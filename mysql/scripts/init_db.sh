#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v mysql)" ]; then
  echo >&2 "Error: mysql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,mysql"
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'mysql'
DB_USER="${MYSQL_USER:=root}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${MYSQL_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'ordersystem'
DB_NAME="${MYSQL_DB:=ordersystem}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${MYSQL_PORT:=3306}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${MYSQL_HOST:=127.0.0.1}"

# Allow to skip Docker if a dockerized mysql database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  # if a mysql container is running, print instructions to kill it and exit
  RUNNING_MYSQL_CONTAINER=$(docker ps --filter 'name=mysql' --format '{{.ID}}')
  if [[ -n $RUNNING_MYSQL_CONTAINER ]]; then
    echo >&2 "there is a mysql container already running, kill it with"
    echo >&2 "    docker kill ${RUNNING_MYSQL_CONTAINER}"
    exit 1
  fi
  # Launch mysql using Docker
  docker run \
      -e MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
      -e MYSQL_DB=${DB_NAME} \
      -p ${DB_PORT}:3306 \
      -d \
      --name "mysql_$(date '+%s')" \
      docker.io/library/mysql:latest
      # ^ Increased maximum number of connections for testing purposes
fi

# Keep pinging MYSQL until it's ready to accept commands
until MYSQL_PWD=${DB_PASSWORD} mysql -h${DB_HOST} -u${DB_USER} -P${DB_PORT} -e 'show global variables like "max_connections";'; do
  >&2 echo "mysql is still unavailable - sleeping"
  sleep 1
done

>&2 echo "MySQL is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "mysql has been migrated, ready to go!"
