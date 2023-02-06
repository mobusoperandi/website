<!-- TOC -->
# Table of contents

1. [Table of contents](#table-of-contents)
1. [Requirements](#requirements)
1. [Local development static web server](#local-development-static-web-server)
1. [Watch for changes and regenerate website when they occur](#watch-for-changes-and-regenerate-website-when-they-occur)
<!-- TOC -->

# Requirements

1. The [Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) specified in `rust-toolchain.toml`.
1. [cargo-run-bin](https://crates.io/crates/cargo-run-bin).
1. [Node.js](https://nodejs.org) and an `$ npm install`.

# Get help from the CLI

```
$ cargo run -- --help
```

And wait for quite a while, even though there might be no progress output.

If you're not certain whether compilation is in progress, consider examining your list of running processes and their CPU usage.

