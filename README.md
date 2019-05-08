# Rust Frankfurt meetup

Repository for the rust frankfurt meetup group: [https://www.meetup.com/Rust-Frankfurt/](https://www.meetup.com/Rust-Frankfurt/)

## setup

Rust setup, as recommended by the rust website:
 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### docker based development

In case you don't want to install rust on your machine, or the setup does not work for you, you can try the docker based setup (requires your system to have [docker](https://www.docker.com/) installed).

Use `make new` to create a new rust app using cargo,

`make run` to build and run it,

`make test` to run cargo test.

Change the `APP_NAME` environment variable accordingly.


## Tutorials

- [Rustlings](https://github.com/rust-lang/rustlings/) - Small exercises to get you used to reading and writing Rust code! 
- [Rust Playground](https://play.rust-lang.org/) - experiment online with a rust runtime
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - a collection of runnable examples that illustrate various Rust concepts and standard libraries

## Books
- [Rust-Book](https://doc.rust-lang.org) 



## useful tools

- clean up the cargo cache: https://github.com/matthiaskrgr/cargo-cache
- format rust code: https://github.com/rust-lang/rustfmt
- watch for changes and run commands: https://github.com/passcod/cargo-watch
  - e.g. use `cargo watch -x fmt -x run` to automatically format and run on any change


## rust 0x02

- Niko was so kind to bring his small sample program: https://github.com/okin/opsi-rpc-rust
  - we extended it from a simple command line parser, to use clap - https://crates.io/crates/clap which allows to parse command line options with a lot of luxury
  - also, we looked into how we can use strong typing for JSON data, exploring the functionality of https://github.com/serde-rs/json

- we also explored some microcontroller/embedded systems coding in rust, looking at sample code for LED blinking on an stm32 blue pill microcontroller, based on https://github.com/TeXitoi/blue-pill-quickstart
