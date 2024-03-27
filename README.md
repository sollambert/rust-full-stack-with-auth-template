# rust-yew-axum-tauri-desktop template

This template is built using Yew, Tailwinds, Axum, Sqlx, and Tauri.

This project is built to make full stack development with user authentication easily accessible for developers within an entirely Rust ecosystem.

In here you'll find some quick demos to get you started with an entirely Rust web environment!

Authentication/authorization is achieved with JSON Web Tokens. This is achieved in a two-part authentication process the involved the user signing in/creating an account at which point they are issued a requester token for a defined amount of time.

With this token, users are then able to request more tokens to perform authorized actions depending on access level. Through frontend middleware, this process is handled automatically and the authorization request token is placed into request headers as a Bearer auth token whenever a request is sent to the backend.

Web socket authentication functions slightly differently, as the authentication handshake occurs through the first message of a freshly opened websocket instead of being sent as a Bearer auth header.

The specific flavor of SQL is inferred from DATABASE_URL environment variable, however this package does allow for conditionally compiling with explicit support for SQLite and Postgres through their respective features if you would like to use flavor-specific syntax in constructed queries.

## Crates

- `frontend`: Yew frontend app for desktop client.
- `backend`: Axum backend restful and websocket api for desktop client.
- `server`: Axum server side restful and websocket api.
- `types`: Common types shared by frontend/backend/server.
- `tauri`: Tauri app for desktop client.

## Development

Install

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install tauri-cli
cargo install sqlx-cli
```

Run desktop client app

```bash
cargo tauri dev
```

Run server side

```bash
cargo run --bin server
```

Bundle desktop client app

```bash
cargo tauri build
```

Run web frontend
```bash
cd crates/frontend
trunk serve
```

Run migrations
```bash
# Postgres
sqlx migrate run --source migrations/postgres
# SQLite
sqlx migrate run --source migrations/sqlite
```

Revert migrations
```bash
# Postgres
sqlx migrate revert --source migrations/postgres
# SQLite
sqlx migrate revert --source migrations/sqlite
```

## Environment Variables

Required environment variables, these can be stored in a .env file at the top level of the repository if not set as OS environment variables.

```bash
# Database URL, SQLx will infer the database type by URL if not specifying with package feature
DATABASEURL=
# 16 byte salt
PASSWORD_SALT=THISISABADSALT!!
# length in seconds the auth token with access information should live, keep it very short
AUTH_TOKEN_EXPIRE=1
# length in seconds the auth requester token should live, this should be the length of time before someone must authenticate with username/password again
AUTH_REQUEST_TOKEN_EXPIRE=84600
# Private secret used for encrypting/decrypting JWT
AUTH_TOKEN_SECRET=THISISABADSECRET
# Company name to set as the Iss claim in JWTs
COMPANY_NAME=PanuccisPizza
# Company domain to set as the Aud claim in JWTs
COMAPNY_DOMAIN=pannucispizza.slice
```

## Contribute

Feel free to take a look at the current issues in this repo for anything that currently needs to be worked on.

You are also welcome to open a PR or a new issue if you see something is missing or could be improved upon.

## License

MIT
