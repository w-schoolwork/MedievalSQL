#!/usr/bin/env bash
./up.sh
docker compose -f .devcontainer/docker-compose.yml exec -w/ws -u $(id -u) app $@
