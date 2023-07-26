#!/bin/bash

cd images

# TODO: Upload them to dockerhub and then just pull them.
docker build -f Dockerfile-lua --tag lua-runtime .
cd ..

./evaluator
