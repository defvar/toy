#!/usr/bin/env bash

if [ $# != 2 ]; then
    echo "args: [env-file] [mount dir for secret]"
    exit 1
fi

sudo docker container rm -f api-server

#--log-driver=fluentd \
#--log-opt fluentd-address=localhost:24224 \

sudo docker run -d \
--network toy \
-p 3030:3030 \
-p 9030:9030 \
--mount type=bind,source=/"$2",destination=/.keys \
--env-file "$1" \
--name api-server \
toy/console-backend
