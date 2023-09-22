#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

cp deploy/nginx.conf /etc/nginx/sites-enabled/survival.vallentin.conf

nginx -t

service nginx restart
