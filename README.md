Fractal
=======

Fractal is a Gtk+ Matrix.org client written in Rust.

 * Come to talk with us in Matrix: https://matrix.to/#/#fractal-gtk:matrix.org
 * Main repository: https://gitlab.gnome.org/danigm/fractal/

![screenshot](https://gitlab.gnome.org/danigm/fractal/raw/master/screenshots/fractal.png)

## How to Build

You need meson and ninja to build this project. Rust and cargo are also
needed.

```
./configure --prefix=/usr/local
make
sudo make install
```

On MacOS, you will need to:
```
brew install gtk3+ dbus bash
# empirically needs 3.22.19 or later of gtk3+
# ...and run configure as:
/usr/local/bin/bash -c ./configure --prefix=/usr/local
```

You may also need to comment out the `notification.show` block in
`./fractal-gtk/src/app.rs` as apparently `notification.wait_for_action`
is missing on MacOS.

## Supported m.room.message (msgtypes)

msgtypes          | Recv                | Send
--------          | -----               | ------
m.text            | Done                | Done
m.emote           |                     |
m.notice          |                     |
m.image           | Done                | Done
m.file            | Done                | Done
m.location        |                     |
m.video           | Done                | Done
m.audio           | Done                | Done

Full reference in: https://matrix.org/docs/spec/client\_server/r0.2.0.html#m-room-message-msgtypes

The origin of Fractal
---------------------

This project is based on ruma-gtk https://github.com/jplatte/ruma-gtk

Instead of using RUMA Client, Fractal calls directly to the matrix.org
REST API.

The first version of this project was called guillotine, based on french revolution,
in relation with the Riot client name, but it's a negative name so we decide
to change for a math one.

The name Fractal was proposed by **Regina Bíró**.
