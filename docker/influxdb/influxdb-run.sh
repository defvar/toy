#!/usr/bin/env bash

mkdir -p /var/influxdb-data
sudo docker container rm -f influxdb
sudo docker run -d \
--network toy \
--name influxdb \
-p 8086:8086 \
--mount type=bind,source=/var/influxdb-data,destination=/var/lib/influxdb2 \
quay.io/influxdb/influxdb:v2.0.7
