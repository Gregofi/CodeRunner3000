#!/bin/bash

set -e

pushd images
docker build -f Dockerfile --tag base-runtime-alpine .

# Specify further images here, all must inherit from base-runtime-alpine:
# docker build -f lua5.1/Dockerfile --tag lua-runtime lua5.1
popd

./evaluator
