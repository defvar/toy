cargo rustc -p toy-pack-derive --example derive  -- -Z unstable-options --pretty=expanded > %~dp0expanded_example.rs
