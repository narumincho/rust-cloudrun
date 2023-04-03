FROM rust:latest AS build

WORKDIR /workspace/rust-cloudrun/

COPY . .

RUN cargo run -p client_build

RUN cargo build -p server --release --all-features

FROM ubuntu:latest

COPY --from=build /workspace/rust-cloudrun/target/release/ /app/

ENV PORT=8000

CMD /app/server
