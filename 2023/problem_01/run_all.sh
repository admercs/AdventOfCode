#!/bin/bash

BENCHMARK=false
VALIDATE=false
SOLUTION=0

DIRNAME="$(basename $(realpath $(dirname $0)))"
echo "Running comparison: $DIRNAME"

# compile C++ and Rust code
(cd ./cpp && make all)
(cd ./rust && cargo build --release)

if [ $VALIDATE = true ]; then
    PYTHON="$(cd ./python && python main.py)"
    JULIA="$(cd ./julia && julia main.jl)"
    CPP="$(cd ./cpp && ./main)"
    RUST="$(cd ./rust && ./main)"

    if [ $PYTHON = $SOLUTION && $JULIA = $SOLUTION && $CPP = $SOLUTION && $RUST = $SOLUTION ]; then
        echo 'Validation succeeded.'
    else
        echo 'Validation failed.'
    fi
fi

if [ $BENCHMARK = true ]; then
    PYTHON="$(cd ./python && time 'python main.py')"
    JULIA="$(cd ./julia && time 'julia main.jl')"
    CPP="$(cd ./cpp && make all && time './main')"
    RUST="$(cd ./rust && cargo build --release && time './main')"
fi

echo 'Comparison complete.'
