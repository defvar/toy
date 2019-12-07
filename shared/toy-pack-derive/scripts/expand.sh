#!/bin/bash

cargo rustc -p toy-pack-derive --example derive  -- -Z unstable-options --pretty=expanded > expanded_example.rs
exit 0
