# Build stage
FROM rust:latest AS BUILD
WORKDIR /usr/src/pepperoni_bot
COPY . .
RUN cargo install --path .

# Package stage
FROM  debian:bullseye AS PACKAGE
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /workdir
COPY --from=BUILD /usr/local/cargo/bin/pepperoni_bot .
COPY --from=BUILD /usr/src/pepperoni_bot/.env .
CMD ["./pepperoni_bot"]
