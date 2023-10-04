#!/usr/bin/env bash
./up.sh
export $(cat .devcontainer/.env | xargs)
export DB_HOSTNAME=localhost
./mkdiagram.sh
./down.sh