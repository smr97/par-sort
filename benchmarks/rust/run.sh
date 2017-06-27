#!/bin/bash

cd $(readlink -f "$(dirname "$0")")
set -ex

cargo run --release
