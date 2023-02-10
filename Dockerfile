FROM rust:latest AS build

WORKDIR /workspace/rust-cloudrun/

COPY . .

RUN cargo build -p server --release --all-features

FROM ubuntu:latest

COPY --from=build /workspace/rust-cloudrun/target/release/ /app/

ENV PORT=8000

CMD /app/server
