#!/usr/bin/env bash

if [ $# != 2 ]; then
    echo "args: [env-file] [mount dir for secret]"
    exit 1
fi

sudo docker container rm -f console-backend

sudo docker run -d \
--network toy \
-p 127.0.0.1:3030:3030 \
--mount type=bind,source=/"$2",destination=/.keys \
--env-file "$1" \
--name console-backend \
toy/console-backend
