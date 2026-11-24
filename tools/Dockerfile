FROM rust:1.95-bookworm
RUN rustup component add clippy rustfmt && rustup target add wasm32-unknown-unknown \
 && chmod -R a+rwX /usr/local/rustup /usr/local/cargo
