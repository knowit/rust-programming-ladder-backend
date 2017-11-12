# rust-programming-ladder-backend

New programming-ladder backend written in Rust.

## Goals

* Write a small and semi serious web service in Rust
* Be the backend of https://github.com/knowit/elm-programming-ladder
* Use GraphQL!
* Use https://github.com/SergioBenitez/Rocket
* Use https://github.com/graphql-rust/juniper
* Integrate with some database

## Installation	

* Install rust: `curl https://sh.rustup.rs -sSf | sh`
* Install nightly toolchain: `rustup toolchain install nightly`
* Clone project and `cd` into project directory
* Set nightly toolchain as override for directory: `rustup override set nightly`
* Launch app with `cargo run`
* Visit `localhost:8000` to confirm that the app is running

## API instability risk
The current state of Rust for the Web is that everything is extremely bleeding edge, with differnt APIs and frameworks appearing and changing very often, and old ones die (reminiscent of the JS world?).

Currently, a typical Rust web server is synchronous, which is not a scalable architecture for a web server that is running on system threads. Therefore, the most recent version of https://github.com/hyperium/hyper HTTP library (0.11) has been rewritten to be based on https://github.com/tokio-rs/tokio, Rust's async IO library (link: http://seanmonstar.com/post/161786147642/hyper-v011). Libraries like `rocket` and `juniper`, which are completely synchronous in their design, may become outdated or API may change significantly in the near future.

We need to keep this in mind when developing this backend.

#### Issues in various dependency libraries discussing the introduction of async
* https://github.com/SergioBenitez/Rocket/issues/17
* https://github.com/graphql-rust/juniper/issues/2
