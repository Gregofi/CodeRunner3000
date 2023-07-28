#!/bin/sh

cd /eval_env &&
# Run the actual code as non-root user
adduser -D evaluator &&
sudo -u evaluator lua5.1 /usr/bin/evaluate.lua source.lua > stdout.txt 2> stderr.txt
