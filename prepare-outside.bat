:: I have no idea if this will work
docker compose -f .devcontainer/docker-compose.yml up -d
docker compose -f .devcontainer/docker-compose.yml exec -w/ws -u 0 app chown -R $(id -u) /ws
docker compose -f .devcontainer/docker-compose.yml exec -w/ws -u $(id -u) app bash -c 'cargo sqlx migrate run && cargo sqlx prepare'
docker compose -f .devcontainer/docker-compose.yml down
