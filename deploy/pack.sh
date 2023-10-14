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
rm -f deploy.tar.gz

echo "Building..."
npm run build

echo "Compressing static files..."
# Ignoring already compressed files and MP4 files (which are also already heavily compressed)
for IN_FILE in $(find static -type f -not -name "*.mp4" -not -name "*.gz"); do
    OUT_FILE="${IN_FILE}.gz"
    echo "Compressing \`$IN_FILE\` -> \`$OUT_FILE\`"

    gzip --keep -f -c "${IN_FILE}" > "${OUT_FILE}"

    IN_SIZE=$(stat -c%s "${IN_FILE}")
    OUT_SIZE=$(stat -c%s "${OUT_FILE}")

    if [ "$OUT_SIZE" -ge "$IN_SIZE" ]; then
        echo "Removing larger compressed file \`$OUT_FILE\`"
        rm -f "${OUT_FILE}"
    fi
done

echo "Packing..."
tar -v -czf deploy.tar.gz \
    static \
    deploy/cert.sh \
    deploy/install.sh \
    deploy/nginx.conf
