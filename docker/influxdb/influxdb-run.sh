#!/usr/bin/env bash

mkdir -p /tmp/influxdb-data.tmp
sudo docker container rm -f  influxdb
sudo docker run -d \
--network toy \
--name influxdb \
-p 8086:8086 \
--mount type=bind,source=/tmp/influxdb-data.tmp,destination=/root/.influxdbv2 \
quay.io/influxdb/influxdb:v2.0.3
