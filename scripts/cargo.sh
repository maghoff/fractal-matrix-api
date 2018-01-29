#!/bin/sh

export CARGO_HOME=$1/target/cargo-home

if [[ $DEBUG = true ]]
then
    echo "DEBUG MODE"
    cargo build -p fractal-gtk && cp $1/target/debug/fractal-gtk $2
else
    echo "RELEASE MODE"
    cargo build --release -p fractal-gtk && cp $1/target/release/fractal-gtk $2
fi
