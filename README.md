# Ichiba API

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![CI](https://github.com/0xYami/ichiba/actions/workflows/ci.yml/badge.svg)

This is the [Ichiba](https://github.com/0xYami/ichiba-ui) HTTP server built in Rust using the Axum framework.

## Prerequisites

Before running the server, make sure [Rust](https://www.rust-lang.org/fr) installed on your system.

- Rust `>=1.72.0`

## Getting Started

Follow these steps to get the server up and running:

Create a `.env` file at the root with the following

```bash
# Server
PORT=
HOST=

# Database
DATABASE_URL=
DATABASE_MAX_CONNECTIONS=

# CORS
CORS_ALLOWED_ORIGIN=
CORS_ALLOWED_METHODS=
CORS_ALLOWED_HEADERS=

# JWT
JWT_SECRET=
JWT_EXPIRATION=
```

Build and run the server:

```bash
$ cargo run
```

The server should now be running at `http://localhost:<port>` where `port` is the value you specified in the `.env` file.

## Dependencies

The server uses the following dependencies:

- `axum`: A web framework for Rust.
- `jsonwebtoken`: A library for handling JWT tokens.
- `serde`: A serialization/deserialization library for Rust.
- `tokio`: An asynchronous runtime for Rust.
- `sqlx`: A database toolkit for Rust.

For more details, refer to the `Cargo.toml` file.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use and modify it according to your needs.
