# Build stage
FROM rust:1.49-slim-buster AS BUILD
WORKDIR /usr/src/pepperoni_bot

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY ./src ./src

# Actual build
RUN rm ./target/release/deps/pepperoni_bot*
RUN cargo build --release

# Package stage
FROM rust:1.49-slim-buster AS PACKAGE

# Copy binary
COPY --from=BUILD /usr/src/pepperoni_bot/target/release/pepperoni_bot .

CMD ["./pepperoni_bot"]
