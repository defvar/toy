#!/bin/bash

SCRIPT_DIR=$(cd $(dirname "$0");pwd)

sudo docker build -t toy/console-backend -f "$SCRIPT_DIR"/Dockerfile "$SCRIPT_DIR"/../../
