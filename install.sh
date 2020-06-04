#!/bin/bash
curl https://sh.rustup.rs > rust.sh # Download Rust installation
chmod +x rust.sh
./rust.sh -y # Runs the installation file
rm rust.sh # Removes the installation file
sudo apt-get install pkg-config libssl-dev # Installs some dependencies
cargo build # Compiles the code
mv target/debug/RBust rbust # Moves the executable to RBust directory
