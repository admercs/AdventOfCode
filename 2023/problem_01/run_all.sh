#!/bin/bash

DIRNAME="$(basename $(realpath $(dirname $0)))"
echo "Running comparison: $DIRNAME"

(cd ./python && python solution.py)
(cd ./julia  && julia solution.jl)
(cd ./cpp    && ./solution)
#(cd ./rust   && ./solution)

echo 'Comparison complete.'
