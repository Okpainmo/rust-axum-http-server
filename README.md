# rust-axum-http-server

A standard monolith http server built with Rust(and the Axum framework).

## Features

1. 4 end-points: `/register`, `/login`, `/logout`, `/upload-profile-image`, and `/delete-profile`.
2. Secure with complete JWT authentication.
3. Sending emails.
4. Web Socket implementation - system-wide alerts on new registrations.
5. DB: Postgres.
6. Docker.

## How to run the project/server.

1. Clone this repository.

```shell
git clone https://github.com/Okpainmo/rust-axum-http-server.git
```

2. Install all the dependencies(and compile code-base).

```shell
cargo build
```

or install latest versions individually

```shell
cargo add axum tokio --features tokio/full serde --features serde/derive serde_json dotenvy sqlx argon2 rand
```

3. Start the local database via Docker

```shell
docker run -d --name ramhs__dev-db -p 5433:5432 -e POSTGRES_USER=okpainmo -e POSTGRES_PASSWORD=supersecret -e POSTGRES_DB=rust-axum-monolith-http-server__db_dev postgres
```

3. Run the server

```shell
cargo run
```

or with `cargo-watch`

```shell
cargo install cargo-watch
```

```shell
cargo watch -q -c -w src/ -x "run"
```

Cheers!!!