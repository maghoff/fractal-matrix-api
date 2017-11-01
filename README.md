Fractal
=======

Fractal is a Gtk+ Matrix.org client written in Rust.

This project is based on ruma-gtk https://github.com/jplatte/ruma-gtk

Instead of using RUMA Client, Fractal calls directly to the matrix.org
REST API.

![screenshot](https://gitlab.gnome.org/danigm/fractal/raw/master/screenshots/fractal.png)

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
