#!/bin/sh

set -e

if [ "$1" = "--release" ]; then
    cmake -B build -D CMAKE_BUILD_TYPE=Release
else
    cmake -B build -D CMAKE_BUILD_TYPE=Debug
fi

cmake --build build
