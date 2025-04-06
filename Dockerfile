# ---- Build Stage ----
FROM rust:1.75 as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo fetch
RUN cargo build --release || true

# Copy actual source
COPY . .

RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/our_top_clinic_backend /app/our_top_clinic_backend

ENV ROCKET_ENV=production
# Set any other environment variables here (DATABASE_URL, etc.)

CMD ["./our_top_clinic_backend"]