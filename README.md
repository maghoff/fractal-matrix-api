# Fractal

Fractal is a Matrix messaging app for GNOME written in Rust. Its interface is optimized for collaboration in large groups, such as free software projects.

* Come to talk to us on Matrix: <https://matrix.to/#/#fractal-gtk:matrix.org>
* Main repository: <https://gitlab.gnome.org/World/fractal/>

![screenshot](https://gitlab.gnome.org/World/fractal/raw/master/screenshots/fractal.png)

## Installation instructions

You can find Fractal installation instructions through packages on the [GNOME wiki](https://wiki.gnome.org/Apps/Fractal).

## Build Instructions

You need Meson and Ninja (as well as Rust and Cargo) to build Fractal.

### GNU/Linux

```sh
meson . _build --prefix=/usr/local
ninja -C _build
sudo ninja -C _build install
```

### macOS

```sh
brew install gtk+3 dbus bash adwaita-icon-theme
# empirically needs 3.22.19 or later of gtk3+
# ...and run configure as:
/usr/local/bin/bash -c "meson . _build --prefix=/usr/local"
ninja -C _build
sudo ninja -C _build install
```

## Supported m.room.message (msgtypes)

msgtypes          | Recv                | Send
--------          | -----               | ------
m.text            | Done                | Done
m.emote           | Done                | Done
m.notice          |                     |
m.image           | Done                | Done
m.file            | Done                | Done
m.location        |                     |
m.video           | Done                | Done
m.audio           | Done                | Done

Full reference in: <https://matrix.org/docs/spec/client\_server/r0.2.0.html#m-room-message-msgtypes>

## The origin of Fractal

This project is based on Fest <https://github.com/fest-im/fest>, formerly called ruma-gtk.

Instead of using RUMA Client, Fractal calls directly to the matrix.org
REST API.

The first version of this project was called guillotine, based on French revolution,
in relation with the Riot client name, but it's a negative name so we decide
to change for a math one.

The name Fractal was proposed by Regina Bíró.