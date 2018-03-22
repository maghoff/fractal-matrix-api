#!/bin/sh

VERSION=$1
DEST=${MESON_BUILD_ROOT}
DIST=$DEST/dist/$VERSION


cd "${MESON_SOURCE_ROOT}"
mkdir -p $DIST

# copying files
cp -rf fractal-gtk $DIST
cp -rf fractal-api $DIST
cp Cargo.toml $DIST
cp Cargo.lock $DIST
cp configure $DIST
cp meson.build $DIST
cp fractal.doap $DIST
cp LICENSE.txt $DIST
cp README.md $DIST
cp -rf scripts $DIST

# cargo vendor
mkdir $DIST/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $DIST/.cargo/config
cp -rf vendor $DIST/

# packaging
cd $DEST/dist
tar -cJvf $VERSION.tar.xz $VERSION
