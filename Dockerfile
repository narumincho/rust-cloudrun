FROM rust:latest as build

WORKDIR /workspace/rust-cloudrun/

COPY . .

RUN cargo build --release --all-features

CMD /workspace/rust-cloudrun/target/release/rust-cloudrun
