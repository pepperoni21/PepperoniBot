# Build stage
FROM rust:1.69-slim AS BUILD
WORKDIR /usr/src

RUN USER=root cargo new pepperoni_bot

RUN USER=root apt update && apt install -y musl-tools

COPY Cargo.toml Cargo.lock /usr/src/pepperoni_bot/

WORKDIR /usr/src/pepperoni_bot

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

COPY src /usr/src/pepperoni_bot/src/

RUN touch /usr/src/pepperoni_bot/src/main.rs

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.17.3 AS RUNTIME

COPY --from=BUILD /usr/src/pepperoni_bot/target/x86_64-unknown-linux-musl/release/pepperoni_bot /usr/local/bin

COPY .env .env

CMD ["/usr/local/bin/pepperoni_bot"]