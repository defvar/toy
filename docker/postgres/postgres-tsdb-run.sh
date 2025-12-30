#!/usr/bin/env bash

mkdir -p /var/tsdbdata
sudo docker container rm -f timescaledb
sudo docker run -d \
--network toy \
-p 5432:5432 \
-v /var/tsdbdata:/var/lib/postgresql \
--name timescaledb \
-e POSTGRES_PASSWORD=postgres \
timescale/timescaledb:latest-pg18
