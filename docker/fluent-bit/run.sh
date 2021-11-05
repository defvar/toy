#!/usr/bin/env bash

if [ $# != 2 ]; then
    echo "args: [env-file] [mount dir for secret]"
    exit 1
fi

ENV_FILE="$1"
SECRET="$2"

sudo docker container rm -f toy-fluent-bit

sudo docker run -d \
--network toy \
-p 24224:24224 \
--mount type=bind,source=/"$SECRET",destination=/.keys \
--name toy-fluent-bit \
--env-file "$ENV_FILE" \
toy/fluent-bit
