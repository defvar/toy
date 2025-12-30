#!/usr/bin/env bash

if [ $# != 3 ]; then
    echo "args: [env-file] [mount dir for secret] [name]"
    exit 1
fi

ENV_FILE="$1"
SECRET="$2"
NAME="$3"

#--log-driver=fluentd \
#--log-opt fluentd-address=localhost:24224 \

sudo docker run -d \
--network toy \
-p 3031:3031 \
-p 9031:9031 \
--mount type=bind,source=/"$SECRET",destination=/.keys \
--env TOY_ACTOR_NAME="$NAME" \
--env-file "$ENV_FILE" \
--name "$NAME" \
toy/actor
