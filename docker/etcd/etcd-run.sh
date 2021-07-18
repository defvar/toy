#!/usr/bin/env bash

if [ "$(uname)" == 'Darwin' ]; then
    tag="-arm64"
else
    tag=""
fi

mkdir -p /var/etcd-data
sudo docker container rm -f etcd-server
sudo docker run -d \
--network toy \
-p 2379:2379 \
-p 2380:2380 \
--mount type=bind,source=/var/etcd-data,destination=/etcd-data \
--name etcd-server \
gcr.io/etcd-development/etcd:v3.5.0${tag} \
/usr/local/bin/etcd \
--name s1 \
--data-dir /etcd-data \
--listen-client-urls http://0.0.0.0:2379 \
--advertise-client-urls http://0.0.0.0:2379 \
--listen-peer-urls http://0.0.0.0:2380 \
--initial-advertise-peer-urls http://0.0.0.0:2380 \
--initial-cluster s1=http://0.0.0.0:2380 \
--initial-cluster-token tkn \
--initial-cluster-state new \
--log-level info \
--logger zap \
--log-outputs stderr
