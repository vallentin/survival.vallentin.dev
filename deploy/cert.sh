#!/usr/bin/env bash

set -e

service nginx stop
ufw allow 80

certbot certonly --standalone --noninteractive --agree-tos --cert-name survival.vallentin \
    -d survival.vallentin.dev \
    -d www.survival.vallentin.dev \
    -d survival.vallentin.io \
    -d www.survival.vallentin.io
# certbot renew

ufw delete allow 80
service nginx start
