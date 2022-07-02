#!/usr/bin/env bash

if [ $# != 3 ]; then
    echo "args: [env-file] [mount dir for secret] [name]"
    exit 1
fi

ENV_FILE="$1"
SECRET="$2"
NAME="$3"

sudo docker run -d \
--network toy \
--log-driver=fluentd \
--log-opt fluentd-address=localhost:24224 \
-p 127.0.0.1:3031:3031 \
--mount type=bind,source=/"$SECRET",destination=/.keys \
--env TOY_SUPERVISOR_NAME="$NAME" \
--env-file "$ENV_FILE" \
--name "$NAME" \
toy/supervisor
