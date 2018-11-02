# Rust on balena - Hello world

This is a very simple project that is an example of how to build and run Rust
code on a device that is supported by [balena](https://balena.io).
It showcases using custom Rust version pinning (via the [standard `rust-toolchain` file](https://github.com/rust-lang-nursery/rustup.rs#the-toolchain-file)) and how to create a multi-staged build process, 
that results in an app image that does not have compilation tools embedded.

## Multi-staged build process; using Dockerfile.template

[Dockerfile.template](Dockerfile.template) file builds artifacts in release
mode with optimizations by default and the final image does not contain
build tools, Rust toolchain or the application source code. It contains
the application binary only.

This sample application depends on the [clap](https://crates.io/crates/clap)
crate intentionally to demonstrate build artifacts caching.

## How to use balena's local push

Set your machine name in the `local-push.env` file. Sample value for
Raspberry Pi 3 B+ model:

```bash
export RESIN_MACHINE_NAME=raspberrypi3
```

[List of balena base images](https://balena.io/docs/reference/base-images/base-images/)
in case you have a different device.

Run [local-push.sh](scripts/local-push.sh) script, which:

* generates `Dockerfile` from `Dockerfile.template`
    * replaces `%%RESIN_MACHINE_NAME%%` with `raspberrypi3`
    * replaces `cargo build --release` with `cargo build` (1)
    * replaces `target/release` with `target/debug` (1)
* launches `sudo resin local push -s . --force-build "$@"` (2)

(1) `resin local push` does not support build arguments (yet), we have to replace these
values with `sed`.

(2) `"$@"` allows you to specify device (`local-push.sh 3a2bbb8.local`) or additional
arguments. You can run this script without arguments and you'll be asked on which device
to push the application.
