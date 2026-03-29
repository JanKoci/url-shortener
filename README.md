# URL Shortener

A hobby project built in Rust to learn web APIs, databases, and backend fundamentals by building a URL shortener from scratch.

## Stack

- **[axum](https://github.com/tokio-rs/axum)** — async HTTP framework
- **[sqlx](https://github.com/launchbadge/sqlx)** + **PostgreSQL** — database with compile-time checked queries
- **[tower_governor](https://github.com/benwis/tower-governor)** — rate limiting middleware

## Features

- `POST /shorten` — accepts a long URL and optional `expires_in_seconds`, returns a short code
- `GET /{code}` — redirects to the original URL, increments click count; returns `410 Gone` if expired
- `GET /stats/{code}` — returns click count and metadata for a short URL
- `GET /health` — health check with DB ping
- Rate limiting per IP address
- Link expiration — short URLs can be created with a TTL

## Running locally

1. Start the database:
   ```bash
   docker compose up -d
   ```

2. Run migrations:
   ```bash
   sqlx migrate run
   ```

3. Start the server:
   ```bash
   cargo run
   ```

The server listens on `http://127.0.0.1:3000`.
