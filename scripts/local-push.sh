#!/bin/sh

set -e

SETTINGS_FILE="local-push.env"
DOCKERFILE_TEMPLATE="Dockerfile.template"
DOCKERFILE="Dockerfile"

#
# Initial checks
#

if [ ! -f ${SETTINGS_FILE} ]; then
    echo "Unable to locate ${SETTINGS_FILE} file"
    exit 1
fi

if [ ! -f ${DOCKERFILE_TEMPLATE} ]; then
    echo "Unable to locate ${DOCKERFILE_TEMPLATE} file"
    exit 1
fi

#
# Generate Dockerfile and build application
#

. "${SETTINGS_FILE}"

echo "Generating Dockerfile for ${RESIN_MACHINE_NAME}..."
sed "s/%%RESIN_MACHINE_NAME%%/${RESIN_MACHINE_NAME}/g; s/cargo build --release/cargo build/g; s/target\/release/target\/debug/g" "${DOCKERFILE_TEMPLATE}" > "${DOCKERFILE}"

echo "Building application..."
sudo resin local push -s . \
    --force-build \
    "$@"
