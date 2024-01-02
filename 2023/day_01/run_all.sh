#!/bin/bash

DIRNAME="$(basename $(realpath $(dirname $0)))"
echo "Running comparison: $DIRNAME"

# compile C++ and Rust code
(cd ./cpp && make build)
(cd ./rust && cargo build --release)

# comparison
echo '-------------------------------------------------------------------------'
echo 'Running comparison...'
echo '-------------------------------------------------------------------------'
echo "Python: $(cd ./python && time python main.py)"
echo "Julia: $(cd ./julia && time julia main.jl)"
echo "C++: $(cd ./cpp && time ./main)"
echo "Rust: $(cd ./rust && time ./target/release/main)"
# replacement
echo "Python: $(cd ./python && time python main.py --replace)"
echo "Julia: $(cd ./julia && time julia main.jl --replace)"
echo "C++: $(cd ./cpp && time ./main --replace)"
echo "Rust: $(cd ./rust && time ./target/release/main --replace)"
echo ''
echo '-------------------------------------------------------------------------'
echo 'Comparison complete.'
echo '-------------------------------------------------------------------------'

