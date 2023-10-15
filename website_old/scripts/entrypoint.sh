#!/bin/bash

envsubst '${WEBSITE_PORT}' < /etc/nginx/nginx.conf.template > /etc/nginx/nginx.conf
echo "Starting nginx on port ${WEBSITE_PORT}"
service nginx restart
gunicorn -w 4 website:app
