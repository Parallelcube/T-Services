#!/bin/bash
echo Cpp service compile
gcc --version
pushd ../cpp/host
make clean
make 
popd
pushd ../cpp/worker
make clean
make 
popd