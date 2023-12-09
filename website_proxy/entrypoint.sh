#!/bin/sh -e

envsubst '$WEBSITE_PROXY_NGINX_STATUS_TOKEN' < /etc/nginx/templates/nginx.conf.template > /etc/nginx/nginx.conf
nginx -g 'daemon off;'
