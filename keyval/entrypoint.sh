#!/bin/bash

if [ -z "$CONFIG_NAME" ]; then
  echo "CONFIG_NAME is not set"
  exit 1
fi

perl -pe 's/\$(\w+)/$ENV{$1}/g' /etc/configs/${CONFIG_NAME}.tmpl.conf > /etc/configs/${CONFIG_NAME}.conf
valkey-server /etc/configs/${CONFIG_NAME}.conf
