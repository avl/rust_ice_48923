#!/bin/bash

cargo clean
cargo build
touch src/lib.rs
cargo build
touch src/lib.rs
cargo build


