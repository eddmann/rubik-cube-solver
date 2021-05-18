FROM rust:1.51.0-buster

RUN cargo install wasm-pack --version 0.9.1 \
    && rustup target add wasm32-unknown-unknown
