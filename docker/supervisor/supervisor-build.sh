#!/bin/bash

SCRIPT_DIR=$(cd $(dirname "$0");pwd)

sudo docker build -t toy/supervisor -f "$SCRIPT_DIR"/Dockerfile "$SCRIPT_DIR"/../../
