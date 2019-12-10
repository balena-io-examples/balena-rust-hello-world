# Rust on balena - Hello world!

This is a very simple project that is an example of how to build and run Rust
code on a device that is supported by [balena][balena-link]. It showcases how to
create a multi-staged build process, that results in an app image that does not
have compilation tools embedded.

## Multi-staged build process; using Dockerfile.template

[Dockerfile.template](Dockerfile.template) file builds artifacts in release mode
with optimizations by default and the final image does not contain build tools,
Rust toolchain or the application source code. It contains the application
binary only.

This sample application depends on the [clap][clap] crate intentionally to
demonstrate build artifacts caching.

## balena Setup & Deployment

To get this project up and running, you will need to signup for a balena account
[here][signup-page] and set up an application and device. You'll find full
details in our [Getting Started tutorial][gettingStarted-link].

Once you have downloaded this project, you can `balena push` it using the
[balenaCLI][balena-cli]. This command will package up and push the code to the
balena builders, where it will be compiled and built and deployed to every
device in the application fleet. When it completes, you'll have your Rust code
running on your device and see some logs on your [balenaCloud
dashboard][balena-dashboard].

[balena-link]:https://balena.io/
[clap]:https://crates.io/crates/clap
[signup-page]:https://dashboard.balena-cloud.com/signup
[gettingStarted-link]:https://www.balena.io/docs/learn/getting-started/raspberrypi3/rust/
[balena-cli]:https://www.balena.io/docs/reference/cli/
[balena-dashboard]:https://dashboard.balena-cloud.com/
