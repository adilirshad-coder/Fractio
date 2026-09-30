FROM rust:1.85-slim AS build
WORKDIR /app
COPY Cargo.toml ./
COPY backend ./backend
COPY programs ./programs
COPY migrations ./migrations
RUN cargo build --release -p fractio-backend
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* && useradd --system --uid 10001 fractio
WORKDIR /app
COPY --from=build /app/target/release/fractio-backend /usr/local/bin/fractio-backend
COPY migrations /app/migrations
USER fractio
EXPOSE 8080
CMD ["fractio-backend"]
