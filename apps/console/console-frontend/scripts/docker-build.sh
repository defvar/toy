#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0);pwd)
sudo docker build -t app-console --build-arg BUILD_DATE=`date -u +"%Y-%m-%dT%H:%M:%SZ"` -f $SCRIPT_DIR/../Dockerfile $SCRIPT_DIR/../
