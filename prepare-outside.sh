#!/usr/bin/env bash
./outside.sh cargo sqlx migrate run
./outside.sh cargo sqlx prepare
./down.sh
