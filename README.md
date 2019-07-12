tmst
====

[![Build Status](https://travis-ci.org/remexre/tmst.svg?branch=master)](https://travis-ci.org/remexre/tmst) [![Crates.io](https://img.shields.io/crates/d/tmst.svg)](https://crates.io/crates/tmst) ![Crates.io License](https://img.shields.io/crates/l/tmst.svg)

Simple timesheet software.

Installation
------------

Grab an amd64 Linux binary from [Releases](https://github.com/remexre/tmst/releases) or build from source using `cargo install tmst`.

Usage
-----

```console
$ tmst in foo
$ tmst current
foo
$ # do some work...
$ tmst out
$ tmst list
1234-56-78
==========

foo - 0.15h

Total: 0.15h
```

GUI Usage
---------

Some scripts to integrate with my i3 setup are included.

Copy or symlink the scripts in [`scripts`](https://github.com/remexre/tmst/tree/master/scripts) to somewhere in your PATH.

Depends on [rofi](https://github.com/davatorium/rofi) and [libnotify](https://github.com/GNOME/libnotify) (for notify-send). Rofi can be replaced by [dmenu](https://tools.suckless.org/dmenu/), see inside `tmst-gui.sh` for details.

Add the following to your i3 config:

```
bindsym $mod+t exec --no-startup-id tmst-gui.sh
```

Add the following to your i3blocks config:

```
[tmst]
command=~/.local/bin/tmst-i3block.sh
interval=10
signal=10
```

`$mod+t` clocks into/out of a project, and while clocked in, the project and the current time will be shown in the i3 bar.

License
-------

Licensed under either of

-	Apache License, Version 2.0, in LICENSE-APACHE
-	MIT License, in LICENSE-MIT

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
