#!/usr/bin/env bash

export DB_HOSTNAME=${DB_HOSTNAME:-$(echo "$DATABASE_URL" | cut -d@ -f2 | cut -d/ -f1)}
echo $DB_HOSTNAME
export DB_IP=$(dig $DB_HOSTNAME | grep db. | tail -n1 | cut -d '	' -f 7)
echo $DB_IP
export ARGS="--server=postgresql --database=${POSTGRES_DB} --host=${DB_IP} --user=${POSTGRES_USER} --password=${POSTGRES_PASSWORD} --command=schema -Fsvg --info-level=standard -o /home/schcrwlr/out.svg"
echo "$ARGS"
docker run -u $(id -u) -v $(pwd):/src:rw --rm --network=host schemacrawler/schemacrawler bash -c "/opt/schemacrawler/bin/schemacrawler.sh $ARGS && cat /home/schcrwlr/out.svg" > diagram.svg

perl -i -pe 'BEGIN{undef $/;} s/<text.+?generated by.+?polygon.*?\/>//smg' diagram.svg
perl -i -pe 's/SchemaCrawler_Diagram/Schema Diagram/smg' diagram.svg
