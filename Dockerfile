FROM ubuntu:latest

COPY ./target/release/ /app/

ENV PORT=8000

CMD /app/server
