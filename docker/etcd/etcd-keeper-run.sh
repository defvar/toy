#!/usr/bin/env bash

# http://localhost:2381/etcdkeeper/

sudo docker container rm -f  etcdkeeper
sudo docker run -d -p 2381:8080 --network toy --name etcdkeeper evildecay/etcdkeeper
