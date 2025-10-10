#!/usr/bin/env bash

# Set up the environment to enable a locally-built docker image to be run locally.
#
# The staging environment also requires some secrets to be set via environment variables:
#
# APP_REDIS_URI
# APP_EMAIL_CLIENT__AUTHORIZATION_TOKEN
# APP_DATABASE__DATABASE_NAME
# APP_DATABASE__HOST
# APP_DATABASE__PORT
# APP_DATABASE__USERNAME
# APP_DATABASE__PASSWORD
#
# And some additional non-secret env vars:
#
# TODO: Just set these on the 'docker run' command line?
#
# RUST_BACKTRACE=full
# APP_ENVIRONMENT=staging
#
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
