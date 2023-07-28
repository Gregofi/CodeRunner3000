#!/bin/bash

cd images

# TODO: Upload them to dockerhub and then just pull them.
docker build -f lua/Dockerfile-lua --tag lua-runtime lua
cd ..

./evaluator
