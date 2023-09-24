#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"

cd /home/vallentin
rm -rf survival.vallentin
mkdir -p survival.vallentin

tar -xzf deploy.tar.gz -C survival.vallentin
rm deploy.tar.gz

cd survival.vallentin

cp deploy/nginx.conf /etc/nginx/sites-enabled/survival.vallentin.conf

nginx -t
service nginx restart
