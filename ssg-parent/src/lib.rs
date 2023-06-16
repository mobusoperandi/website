#![deny(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

mod dev;
mod rebuild_and_run;
mod server;

pub use dev::dev;
