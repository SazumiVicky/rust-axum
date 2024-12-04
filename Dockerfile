FROM rust:1.73-slim-bullseye as builder
WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

RUN cargo build --release
RUN rm src/main.rs
COPY src src/
COPY static static/
RUN cargo build --release
FROM debian:bullseye-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/simple-api ./app
COPY --from=builder /usr/src/app/static ./static
EXPOSE 3000
ENV RUST_LOG=info
CMD ["./app"] 
