#!/bin/sh

set -e

if [ $# -lt 3 ]; then
    echo "Usage: $0 <stdout> <stderr> <command>..."
    exit 1
fi

if [ "$1" = "/dev/null" ]; then
    STDOUT_FILE="${1}"
else
    STDOUT_FILE="/evaluator/mounted/${1}"
fi
shift

if [ "$1" = "/dev/null" ]; then
    STDERR_FILE="${1}"
else
    STDERR_FILE="/evaluator/mounted/${1}"
fi
shift

adduser -s /bin/false -D -h "/home/${USER_NAME}" "${USER_NAME}" -u "${USER_UID}"

cp /evaluator/mounted/source /home/evaluator_nobody/
cd /home/evaluator_nobody

# chmod +rw /home/evaluator_nobody -R

set +e
sudo -u evaluator_nobody $@ >> "${STDOUT_FILE}" 2>> "${STDERR_FILE}"
exit_code=${?}
set -e

exit ${exit_code}
