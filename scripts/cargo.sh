#!/bin/sh

cargo build --release && cp $1/target/release/fractal $2
