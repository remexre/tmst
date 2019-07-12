tmst
====

[![Build Status](https://travis-ci.org/remexre/tmst.svg?branch=master)](https://travis-ci.org/remexre/tmst) [![Crates.io](https://img.shields.io/crates/d/tmst.svg)](https://crates.io/crates/tmst) ![Crates.io License](https://img.shields.io/crates/l/tmst.svg)

Simple timesheet software.

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

Installation
------------

Grab an amd64 Linux binary from [Releases](https://github.com/remexre/tmst/releases) or build from source using `cargo install tmst`.

License
-------

Licensed under either of

-	Apache License, Version 2.0, in LICENSE-APACHE
-	MIT License, in LICENSE-MIT

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
