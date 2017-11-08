#!/bin/bash

flatpak-builder --repo=repo rust-sdk org.freedesktop.Sdk.Extension.rust.json
flatpak-builder --repo=repo fractal org.gnome.Fractal.json

#flatpak --user remote-add --no-gpg-verify --if-not-exists fractal-repo repo
#flatpak --user install fractal-repo org.freedesktop.Sdk.Extension.rust
#flatpak --user install fractal-repo org.gnome.Fractal
