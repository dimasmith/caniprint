# Cross-compilation

This project can be cross-compiled for the Raspberry Pi (64-bit).

## Prerequisites

- [Docker](https://www.docker.com/)
- [Cross](https://github.com/cross-rs/cross)

Install `cross` with `cargo install cross`.

Make sure you have Docker running.

## Cross-compilation

Build the project for the Raspberry Pi with the following command:

```bash
cross build --target aarch64-unknown-linux-gnu --release
```