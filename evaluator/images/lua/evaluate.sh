#!/bin/sh

TIMEOUT_EXIT_CODE=137

cd /eval_env &&
# Run the actual code as non-root user
adduser -D evaluator &&
timeout -s KILL ${EVALUATOR_TIMEOUT:-5} sudo -u evaluator lua5.1 /usr/bin/evaluate.lua source.lua > stdout.txt 2> stderr.txt
if [ $? -eq ${TIMEOUT_EXIT_CODE} ]; then
    echo "The program timeouted"
    echo "The program timeouted" > stderr.txt
fi
