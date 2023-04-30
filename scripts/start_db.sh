#!/usr/bin/env sh

docker stop budget-db && docker rm budget-db

docker run \
    -p 5432:5432 \
    -e POSTGRES_USER=budget_user \
    -e POSTGRES_PASSWORD=password \
    -d \
    --name budget-db \
    postgres

# Sleep for a second to ensure postgres has stated up.
sleep 1

if [ ! -z ${1+x} ]; then
    echo "Importing from $1"
    cat $1 | docker exec -i budget-db psql -U budget_user
fi

