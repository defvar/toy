#!/usr/bin/env bash

mkdir -p /var/pgdata
sudo docker container rm -f postgres
sudo docker run -d \
--network toy \
-p 5432:5432 \
-v /var/pgdata:/var/lib/postgresql \
--name postgres \
-e POSTGRES_PASSWORD=postgres \
postgres:18.1
