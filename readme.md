# Instructions

Code example copied from [the original AWS blog post](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/)

## Cargo support for musl target

rustup target add x86_64-unknown-linux-musl

## Install musl-tools

sudo apt install musl-tools

## Build

cargo build --release --target x86_64-unknown-linux-musl

## Package

zip -j target/rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
