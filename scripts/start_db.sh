#!/usr/bin/env sh

docker run \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=password \
    -d \
    postgres
