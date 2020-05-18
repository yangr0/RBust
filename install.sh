#!/bin/bash
curl https://sh.rustup.rs -sSf | sh # Installs Rust
sudo apt-get install pkg-config libssl-dev # Installs some dependencies
cargo build # Compiles the code
mv target/debug/RBust rbust # Moves the executable to RBust directory