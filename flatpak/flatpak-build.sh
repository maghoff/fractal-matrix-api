#!/bin/bash

#flatpak --user install flathub org.freedesktop.Sdk.Extension.rust-stable
flatpak-builder --repo=repo fractal org.gnome.Fractal.json

#flatpak --user remote-add --no-gpg-verify --if-not-exists fractal-repo repo
#flatpak --user install fractal-repo org.gnome.Fractal

#flatpak-builder --run fractal org.gnome.Fractal-local.json fractal
