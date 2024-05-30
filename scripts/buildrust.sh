#!/bin/bash
echo Rust service compile
gcc --version
pushd ../rust
cargo build
popd