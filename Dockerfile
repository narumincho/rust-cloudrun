FROM rust:latest

COPY [".", "."]

RUN ["cargo", "build", "--release", "--all-features"]

CMD [ "./target/release/deno-cloudrun" ]
