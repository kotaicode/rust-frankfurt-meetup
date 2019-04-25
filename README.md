# Rust Frankfurt meetup

Repository for the rust frankfurt meetup group: [https://www.meetup.com/Rust-Frankfurt/](https://www.meetup.com/Rust-Frankfurt/)

## setup

Rust setup, as recommended by the rust website:
 [https://www.rust-lang.org/tools/install](https://github.com/rust-lang/rustlings/)

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