# URL Shortener

A hobby project built in Rust to learn web APIs, databases, and backend fundamentals by building a URL shortener from scratch.

## Stack

- **[axum](https://github.com/tokio-rs/axum)** — async HTTP framework
- **[sqlx](https://github.com/launchbadge/sqlx)** + **PostgreSQL** — database with compile-time checked queries
- **[redis](https://github.com/redis-rs/redis-rs)** — redirect caching to avoid DB reads on every request
- **[tower_governor](https://github.com/benwis/tower-governor)** — rate limiting middleware

## Features

- `POST /shorten` — accepts a long URL and optional `expires_in_seconds`, returns a short code
- `GET /{code}` — redirects to the original URL, increments click count; returns `410 Gone` if expired
- `GET /stats/{code}` — returns click count and metadata for a short URL
- `GET /qr/{code}` — returns a PNG QR code for the original URL
- `GET /health` — health check with DB ping
- Redis caching — redirects are cached, DB reads only happen on first visit
- Rate limiting per IP address
- Link expiration — short URLs can be created with a TTL

## Running locally

1. Start the database and Redis:
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

## Usage examples

**Create a short URL:**
```bash
curl -X POST http://127.0.0.1:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.example.com/some/very/long/path"}'

# {"short_code":"aB3xYz"}
```

**Create a short URL that expires in 60 seconds:**
```bash
curl -X POST http://127.0.0.1:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.example.com", "expires_in_seconds": 60}'

# {"short_code":"kP9mQr"}
```

**Redirect to the original URL:**
```bash
curl -L http://127.0.0.1:3000/aB3xYz
```

**Get click stats:**
```bash
curl http://127.0.0.1:3000/stats/aB3xYz

# {"short_code":"aB3xYz","original_url":"https://www.example.com/some/very/long/path","click_count":5,"created_at":"2026-03-29T10:00:00Z","expires_at":null}
```

**Get a QR code (saves as PNG):**
```bash
curl http://127.0.0.1:3000/qr/aB3xYz --output qr.png && open qr.png
```
