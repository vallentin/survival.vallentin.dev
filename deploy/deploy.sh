#!/usr/bin/env bash

set -e

dirname=$(cd "$(dirname "${BASH_SOURCE[0]}")"; pwd -P)
cd "$dirname"
cd ..

if [ "$TERM_PROGRAM" == "vscode" ]; then
    clear
fi

bash deploy/pack.sh

source ".env" set

echo "Uploading..."
scp -i "${DEPLOY_SSH_KEY}" deploy.tar.gz \
    ${DEPLOY_SSH_USER}@${DEPLOY_SSH_ADDR}:/home/vallentin/deploy.tar.gz

echo "Installing..."
ssh ${DEPLOY_SSH_USER}@${DEPLOY_SSH_ADDR} -i "${DEPLOY_SSH_KEY}" \
    "bash -s" < "deploy/install.sh"

echo "Cleaning locally..."
rm -f deploy.tar.gz
