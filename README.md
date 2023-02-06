<!-- TOC -->
# Table of contents

1. [Table of contents](#table-of-contents)
1. [Requirements](#requirements)
1. [Get help from the CLI](#get-help-from-the-cli)
1. [If there's no output for a while](#if-there's-no-output-for-a-while)
<!-- TOC -->

# Requirements

1. The [Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) specified in `rust-toolchain.toml`.
1. [cargo-run-bin](https://crates.io/crates/cargo-run-bin).
1. [Node.js](https://nodejs.org) and an `$ npm install`.

# Get help from the CLI

```
$ cargo run -- --help
```

# If there's no output for a while

That may be [because of `cargo-run-bin`](https://github.com/dustinblackman/cargo-run-bin/issues/2).
It's compiling executable depenedencies in the background and does not pass the output through.

If you're not certain whether compilation is in progress, consider looking for a `rustc` process.

