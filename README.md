<!-- TOC -->
# Table of contents

1. [Table of contents](#table-of-contents)
1. [Requirements](#requirements)
1. [Local dev server](#local-dev-server)
<!-- TOC -->

# Requirements

1. The [Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) specified in `rust-toolchain.toml`.
1. [cargo-run-bin](https://crates.io/crates/cargo-run-bin).
1. [GitHub CLI](https://cli.github.com/).
1. [Node.js](https://nodejs.org) and an `$ npm install`.

# Local dev server

```
$ cargo bin cargo-make dev
```

And wait for quite a while, even though there might be no progress output.

If you're not certain whether compilation is in progress, consider examining your list of running processes and their CPU usage.

