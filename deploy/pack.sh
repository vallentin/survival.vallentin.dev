#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"
cd ..

if [ "$TERM_PROGRAM" == "vscode" ]; then
    clear
fi

echo "Cleaning..."
rm -rf static

echo "Baking..."
npm run bake

echo "Packing..."
tar -v -czf deploy.tar.gz \
    static \
    deploy/cert.sh \
    deploy/install.sh \
    deploy/nginx.conf
