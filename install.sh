curl https://sh.rustup.rs -sSf | sh
sudo apt-get install pkg-config libssl-dev
cargo build
mv target/debug/RBust .