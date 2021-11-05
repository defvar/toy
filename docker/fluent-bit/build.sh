#!/bin/bash

SCRIPT_DIR=$(cd $(dirname "$0");pwd)
echo "$SCRIPT_DIR"
sudo docker build -t toy/fluent-bit -f "$SCRIPT_DIR"/Dockerfile "$SCRIPT_DIR"
