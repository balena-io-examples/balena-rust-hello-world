# A Simple server with Rust

[![balena deploy button](https://www.balena.io/deploy.svg)](https://dashboard.balena-cloud.com/deploy?repoUrl=https://github.com/balena-io-examples/balena-rust-hello-world)

This is a simple skeleton Rust server project that works on any of the [balena][balena-link] supported devices.

This project serves up a welcome page on port `:80` of your balena device.

To get this project up and running, you will need to signup for a balena account [here][signup-page] and set up an application and device. You'll find full details in our [Getting Started tutorial][gettingStarted-link]. 

To get this project up and running, you will need to signup for a balena account [here][signup-page] and set up an application and device. You'll find full details in our [Getting Started tutorial][gettingStarted-link]. 

Once you have downloaded this project, you can `balena push` it using the [balenaCLI][balena-cli]. This command will package up and push the code to the balena builders, where it will be compiled and built and deployed to every device in the application fleet. When it completes, you'll have a node.js web server running on your device and see some logs on your [balenaCloud dashboard][balena-dashboard].

To give your device a public URL, access the device page on the [balenaCloud dashboard][balena-dashboard], and choose the _Public Device URL_ toggle. Alternatively, you can point your browser to your device's IP address.

[balena-link]:https://balena.io/
[clap]:https://crates.io/crates/clap
[signup-page]:https://dashboard.balena-cloud.com/signup
[gettingStarted-link]:https://www.balena.io/docs/learn/getting-started/raspberrypi3/rust/
[balena-cli]:https://www.balena.io/docs/reference/cli/
[balena-dashboard]:https://dashboard.balena-cloud.com/