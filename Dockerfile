FROM rust:latest AS build

WORKDIR /workspace/rust-cloudrun/

COPY . .

RUN cargo build -p server --release --all-features

FROM scratch

COPY --from=build /workspace/rust-cloudrun/target/release/ /app/

CMD /app/server
