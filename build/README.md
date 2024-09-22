# Cross-compilation

This project can be cross-compiled for the Raspberry Pi (64-bit).

## Prerequisites

- [Docker](https://www.docker.com/)
- [Cross](https://github.com/cross-rs/cross)

Install `cross` with `cargo install cross`.

Make sure you have Docker running.

## Custom image for RPi cross-compilation

Raspbian usually has an older version of the `libc` than arch machines.
The compiled program won't start.

The Raspbian is based on the Debian Bookworm.
The image in the `Dockerfile.cross-raspbian-aarch64` uses the Debian Bookworm as a base image.

Build the image with the following command:

```bash
docker build -t cross-raspbian:aarch64 -f Dockerfile.cross-raspbian-aarch64 .
```

## Cross-compilation

Build the project for the Raspberry Pi with the following command:

```bash
cross build --target aarch64-unknown-linux-gnu --release
```