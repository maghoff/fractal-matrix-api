Guillotine
==========

<img src="https://github.com/danigm/guillotine/blob/master/res/guillotine.png" width="200px"/>

This project is based on ruma-gtk https://github.com/jplatte/ruma-gtk

Instead of using RUMA Client, Guillotine calls directly to the matrix.org
REST API.

![screenshot](https://github.com/danigm/guillotine/blob/master/screenshots/guillotine.png)

## Supported m.room.message (msgtypes)

msgtypes          | Recv                | Send
--------          | -----               | ------
m.text            | Done                | Done
m.emote           |                     |
m.notice          |                     |
m.image           | Done                | Done
m.file            |                     | Done
m.location        |                     |
m.video           |                     | Done
m.audio           |                     | Done

Full reference in: https://matrix.org/docs/spec/client\_server/r0.2.0.html#m-room-message-msgtypes
