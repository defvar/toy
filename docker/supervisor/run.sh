#!/usr/bin/env bash

if [ $# != 2 ]; then
    echo "args: [env-file] [mount dir for secret]"
    exit 1
fi

ENV_FILE="$1"
SECRET="$2"

sudo docker run -d \
--network toy \
--log-driver=fluentd \
--log-opt fluentd-address=localhost:24224 \
--mount type=bind,source=/"$SECRET",destination=/.keys \
--env-file "$ENV_FILE" \
toy/supervisor
