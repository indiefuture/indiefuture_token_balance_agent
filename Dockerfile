# Compile the Rust binary
FROM rust:1.83.0-slim-bullseye AS builder
WORKDIR /app
COPY Cargo.toml /app/
COPY Cargo.lock /app/
 
COPY actions /app/actions/
COPY src /app/src/ 
COPY public /app/public/ 
RUN apt update && apt install -y pkg-config libssl-dev ca-certificates
RUN cargo build --release



FROM debian:bullseye-slim AS webserver_runtime
WORKDIR /app
RUN apt update && apt install -y ca-certificates
COPY --from=builder /app/target/release/webserver /app/webserver
 

#run the app 
ENTRYPOINT ["/app/webserver"]