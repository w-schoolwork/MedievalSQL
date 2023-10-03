#!/usr/bin/env bash
docker compose -f .devcontainer/docker-compose.yml up -d --build
docker compose -f .devcontainer/docker-compose.yml exec -w/ws -u 0 app chown -R $(id -u) /ws
docker compose -f .devcontainer/docker-compose.yml exec -w/ws -u $(id -u) app bash ./recreate-db.sh
docker compose -f .devcontainer/docker-compose.yml down
