#!/usr/bin/env bash

if [ $# != 3 ]; then
    echo "args: [name] [env-file] [mount dir for secret]"
    exit 1
fi

NAME="$1"
ENV_FILE="$2"
SECRET="$3"

sudo docker container rm -f supervisor-"$NAME"

sudo docker run -d \
--network toy \
--mount type=bind,source=/"$SECRET",destination=/.keys \
--env-file "$ENV_FILE" \
--name supervisor-"$NAME" \
toy/supervisor

#subscribe \
#--log /var/log/toy/supervisor.log \
#--name "$NAME"
