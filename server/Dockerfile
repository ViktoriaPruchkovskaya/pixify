FROM rust:1.78.0 as build

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY src ./src
COPY benches ./benches

RUN cargo build --release

FROM ubuntu:latest as app

WORKDIR /app

COPY --from=build /app/target/release/pixify .
CMD ["./pixify"]
