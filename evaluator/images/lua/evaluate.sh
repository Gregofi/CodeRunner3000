#!/bin/sh

TIMEOUT_EXIT_CODE=137

set -e
cd /eval_env 
# Run the actual code as non-root user
adduser -D evaluator
set +e

timeout -s KILL ${EVALUATOR_TIMEOUT:-5} sudo -u evaluator lua5.1 /usr/bin/evaluate.lua source.lua > stdout.txt 2> stderr.txt
exit_code=${?}
echo "Program exit code ${exit_code}"
if [ ${exit_code} -eq ${TIMEOUT_EXIT_CODE} ]; then
    echo "The program timeouted"
    echo "The program timeouted" > stderr.txt
fi
