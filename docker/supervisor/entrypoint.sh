#!/bin/bash

cd /usr/local/bin || exit
exec /usr/local/bin/supervisor-d subscribe --name kakiku
