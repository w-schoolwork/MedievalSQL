#!/usr/bin/env bash

docker compose -f .devcontainer/docker-compose.yml exec db psql -Upostgres -dmsql $@