#!/bin/bash

DIRNAME="$(basename $(realpath $(dirname $0)))"
echo "Running comparison: $DIRNAME"

(cd ./python && python main.py)
(cd ./julia && julia main.jl)
(cd ./cpp && make all && ./main)
(cd ./rust && cargo build --release && ./main)

echo 'Comparison complete.'
