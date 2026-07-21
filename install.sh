#!/bin/sh

# install the base
cargo build --release
sudo cp -f target/release/cantor /usr/bin/cantor

# create the config dir
mkdir -p ~/.config/cantor/
cp -f example.toml ~/.config/cantor/config.toml

cantor &
