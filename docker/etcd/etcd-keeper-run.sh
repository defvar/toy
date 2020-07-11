#!/usr/bin/env bash

sudo docker container rm -f  etcdkeeper
sudo docker run -d -p 2381:8080 --network toy --name etcdkeeper evildecay/etcdkeeper
