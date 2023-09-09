#!/usr/bin/env bash

export DB_HOSTNAME=$(echo "$DATABASE_URL" | cut -d@ -f2 | cut -d/ -f1)
echo $DB_HOSTNAME
export DB_IP=$(dig $DB_HOSTNAME | grep db. | tail -n1 | cut -d '	' -f 7)
echo $DB_IP
export ARGS="--server=postgresql --database=${POSTGRES_DB} --host=${DB_IP} --user=${POSTGRES_USER} --password=${POSTGRES_PASSWORD} --command=schema -Fsvg --info-level=standard -o/src/diagram.svg"
echo "$ARGS"
docker run -v $(pwd):/src:rw -uroot --rm --network=host schemacrawler/schemacrawler /opt/schemacrawler/bin/schemacrawler.sh $ARGS