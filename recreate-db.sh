#!/bin/bash

if [ -z "$DATABASE_URL" ]; then
	>&2 echo "You need to set the \"DATABASE_URL\" environment variable so we know what to log into."
	exit 1
fi

cargo sqlx database drop
cargo sqlx database create
cargo sqlx migrate run
cargo sqlx prepare