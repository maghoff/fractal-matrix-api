#!/bin/sh

cargo build --release -p fractal-gtk && cp $1/target/release/fractal-gtk $2
