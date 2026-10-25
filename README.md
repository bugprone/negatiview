# Negatiview

[![CI](https://github.com/bugprone/negatiview/actions/workflows/ci.yml/badge.svg)](https://github.com/bugprone/negatiview/actions/workflows/ci.yml)

*Why you shouldn't purchase this product.*

Negatiview is a [RealWorld](https://github.com/gothinkster/realworld)-style
publishing platform for negative product reviews, written end to end in Rust.
The server is Axum + SQLx on PostgreSQL with Redis for token revocation; the
client is Yew compiled to WebAssembly and styled with Tailwind.

## Features

- Email/password sign-up and login with RS256 JWTs. Short-lived access tokens
  and longer-lived refresh tokens are issued as HttpOnly cookies and also
  accepted as `Authorization: Bearer`. Every access token is keyed by UUID in
  Redis with a TTL, so a token can be revoked before it expires.
- Posts written in Markdown (rendered client-side with `pulldown-cmark`),
  tagged, favorited, edited and deleted by their author.
- Comments, author profiles, follow/unfollow, and a personal feed built from
  the users you follow.
- Pagination, a popular-tags sidebar, and a global feed for anonymous visitors.
- Single binary deploy: the server serves the compiled client from `--static-dir`
  and falls back to `index.html` for client-side routes.

## Stack

| Layer | Crates |
|---|---|
| Server | `axum` 0.8, `tower-http`, `sqlx` 0.9 (PostgreSQL), `redis` 1.x, `jsonwebtoken` 11, `argon2`, `tokio` |
| Client | `yew` 0.23, `yew-router`, `yew-hooks`, `gloo-net`, `pulldown-cmark`, Tailwind CSS v4 |
| Tooling | Rust 2024 edition (MSRV 1.94), `trunk` (wasm bundler), `sqlx-cli` 0.9 (migrations, offline query cache), Docker Compose |

![Database schema](https://github.com/bugprone/negatiview/assets/17267089/bf708310-bee5-4fbb-b223-e91726f1d4e9)

## API

All routes live under `/api`. Routes marked 🔒 need a valid access token.

| Method | Path | Description |
|---|---|---|
| GET | `/health` | Liveness check |
| POST | `/user/sign_up` | Create an account |
| POST | `/user/login` | Issue access + refresh tokens |
| GET / PUT | `/user` 🔒 | Current user, update profile or password |
| GET | `/profile/{display_name}` 🔒 | Public profile |
| POST / DELETE | `/profile/{display_name}/follow` 🔒 | Follow / unfollow |
| GET / POST | `/posts` 🔒 | List (filter by tag, author, favorited) / create |
| GET | `/posts/feed` 🔒 | Posts from followed users |
| GET / PUT / DELETE | `/posts/{id}` 🔒 | Read / edit / delete a post |
| POST / DELETE | `/posts/{id}/favorite` 🔒 | Favorite / unfavorite |
| GET / POST | `/posts/{id}/comments` 🔒 | List / add comments |
| DELETE | `/posts/{id}/comments/{comment_id}` 🔒 | Delete a comment |
| GET | `/tags` | Popular tags |

## Getting started

Prerequisites: Rust 1.94+, the `wasm32-unknown-unknown` target, `trunk`,
`sqlx-cli`, Node 20+ (for the Tailwind v4 CLI) and Docker.

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk sqlx-cli
(cd client && npm install)   # Tailwind CLI, used by Trunk's pre-build hook
```

1. **Configure.** Copy `.env.example` to `.env` and generate two RSA key pairs,
   one for access tokens and one for refresh tokens. The server reads them
   base64-encoded:

   ```bash
   cp .env.example .env
   openssl genrsa -out access.pem 4096 && openssl rsa -in access.pem -pubout -out access.pub.pem
   base64 -i access.pem  | tr -d '\n'   # -> ACCESS_TOKEN_PRIVATE_KEY
   base64 -i access.pub.pem | tr -d '\n' # -> ACCESS_TOKEN_PUBLIC_KEY
   # repeat for REFRESH_TOKEN_*
   ```

2. **Start PostgreSQL and Redis**, then create the database and run migrations:

   ```bash
   make up
   sqlx database create
   sqlx migrate run
   ```

3. **Run** both halves with hot reload. `dev.sh` starts `trunk serve` on
   http://localhost:8080 and proxies `/api` to the Axum server on port 8081
   (needs `cargo install cargo-watch`). Trunk's pre-build hook runs
   `npm run css` to regenerate `tailwind.css` from `client/input.css`:

   ```bash
   ./dev.sh
   ```

### Production build

```bash
cd client && trunk build --release && cd ..
cargo run --release --bin server -- --port 8080 --static-dir ./dist
```

### Building without a database

SQLx checks every `query!` against a live database at compile time. The
checked query metadata is committed under `.sqlx/`, so a plain
`cargo build` works offline:

```bash
SQLX_OFFLINE=true cargo check --workspace
```

After changing a query, refresh the cache with a database running:

```bash
cargo sqlx prepare --workspace
```

## Project layout

```
client/           Yew application (pages, components, API services, types)
client/input.css  Tailwind v4 entry (plus v3-compat base styles); built by `npm run css`
client/package.json  Tailwind CLI only; Rust is built by trunk
server/
  src/handlers/   Axum handlers per resource (user, profile, post, comment, tag)
  src/middlewares/auth.rs   JWT verification + Redis token lookup
  src/models/     Row types and query helpers
  src/routes.rs   Router, CORS, tracing, static-file fallback
migrations/       SQLx migrations
dev.sh            trunk serve + cargo watch
```

## License

[MIT](LICENSE)
