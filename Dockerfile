FROM rust:latest as build

WORKDIR /app

COPY . .

RUN cargo build --release --all-features

FROM scratch

COPY --from=build /app/target/release/rust-cloudrun /app/

CMD /app/rust-cloudrun
