#!/usr/bin/env bash

set -e

DIR=$(cd -P -- "$(dirname -- "$0")" && pwd -P)

cd $DIR/..

echo "*** Start building parallel ***"

cd $(dirname ${BASH_SOURCE[0]})/..


docker build -t parallelfinance/parallel:latest .
