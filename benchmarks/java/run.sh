#!/bin/bash

cd $(readlink -f "$(dirname "$0")")
set -ex

javac Bench.java
java Bench
