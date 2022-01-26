#! /usr/bin/bash

# tested on 
# kubuntu 20.04 lts


git pull

export PATH=~/.cargo/bin:$PATH

# check if cargo is installed
which cargo
CARGO_INSTALLED=$?

if [ $CARGO_INSTALLED == 1 ]
#install it if not present
then
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
	rustup install stable
	rustup default stable
fi

# build binary in release mode
cargo build --release

# copy binary to /usr/bin dir so it is accessible anywhere
sudo cp target/release/docker-manager /usr/bin

# uninstall rust tool chain if it was not present before this script
if [ $CARGO_INSTALLED == 1 ]
then
	echo "y" | rustup self uninstall
fi
