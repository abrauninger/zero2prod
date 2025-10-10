#!/usr/bin/env bash

# Set up the environment to enable a locally-built docker image to be run locally.
# To run:
#
# docker build .
# source ./scripts/env_staging.sh
# docker run <container name> -p 8000:8000 --name staging
#
# Then run the application by visiting in the browser:
# http://localhost:8000

set -xo pipefail
docker build . -t zero2prod-staging-image
docker run --env-file .env.staging -p 8000:8000 zero2prod-staging-image --name staging
