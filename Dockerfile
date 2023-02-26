# Build stage
FROM rust:slim-buster AS BUILD
WORKDIR /usr/src/pepperoni_bot

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Create a dummy source file to cache dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Remove dummy source file
RUN rm -r src

# Copy source code
COPY ./src ./src

# Actual build
RUN rm ./target/release/deps/pepperoni_bot*
RUN cargo build --release

# Package stage
FROM rust:slim-buster AS PACKAGE

# Copy binary
COPY --from=BUILD /usr/src/pepperoni_bot/target/release/pepperoni_bot .

CMD ["./pepperoni_bot"]
