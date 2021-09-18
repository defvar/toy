#!/bin/bash

cargo rustc -p toy-pack-derive --example derive  -- -Zunpretty=expanded > expanded_example.rs
exit 0
