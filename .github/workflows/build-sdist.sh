#!/bin/bash

set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
export PATH="$HOME/.cargo/bin:$PATH"

echo "$(ls)"

pip install -U setuptools wheel setuptools-rust
python setup.py sdist

echo "$(ls)"
