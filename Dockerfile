FROM rust:1.82.0 AS builder
WORKDIR /usr/src/app

# First, copy manifests to cache dependency layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release || true

# Then, copy the entire source
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/our-top-clinic-backend /usr/local/bin/app
ENV PORT=8080
EXPOSE 8080
CMD ["/usr/local/bin/app"]