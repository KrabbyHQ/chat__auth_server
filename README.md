# Krabby/chat/auth_server.

This repository contains the AUTH REST API layer/service for the Krabby `chat` implementation.

## Core Features

- Complete JWT + Cookie Authentication implementation.

- Argon2 password Hashing.

- PostgreSQL + SQLX for database and database operations respectively.

- Dynamic Multi-layer configuration system (TOML + Environment Variables).

- Robust Error Handling and clear API responses.

- Comprehensive Testing with unit tests for utilities and integration tests for API endpoints.

- Middleware Integration for logging, request timeouts, and Cookie management.

## Setup & Execution

### 1. Core Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

- [Docker](https://www.docker.com/) (for database)

- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)

- [NodeJs](https://nodejs.org/en/download/) and [Bun](https://bun.sh/) (for contribution standards enforcement)

### 2. Database Setup

Start the local PostgreSQL database:


E.g.

```shell
docker run -d --name rusty-chat__dev_db -p 5433:5432 -e POSTGRES_USER=okpainmo -e POSTGRES_PASSWORD=supersecret -e POSTGRES_DB=rusty_chat_db_dev postgres
```

Initialize the schema to sync with your newly set up database:

```shell
sqlx migrate run --database-url postgres://<user-name>:<password>@localhost:5433/<database-name>
```

E.g.

```shell
sqlx migrate run --database-url postgres://okpainmo:supersecret@localhost:5433/rusty_chat_db_dev
```

#### If Contributing

1. Initialize the migration - to add/register a new migration file.

```shell
sqlx migrate add <migration_name>
```

E.g.

```shell
sqlx migrate add added_new_hello_field_to_users_table
```

2. Edit the migration file to add the SQL schema update.

3. Sync the database with the new migration.

```shell
sqlx migrate run --database-url postgres://<user-name>:<password>@localhost:5433/<database-name>
```

### 3. Running the Server

*Ensure to have installed `cargo-watch`.*

```shell
cargo install cargo-watch
```

To start the server in development mode(auto-reload enabled), simply run:

```shell
cargo dev
```

> `cargo-watch` handles the server/project reloads on-save. See `.config/config.toml` for reference on the `dev` command.

*Note: The `dev` command is an alias for `cargo watch`. If you are on WSL and reload doesn't trigger, proceed to use the polling command option(also see `.cargo/config.toml` for reference on that).*

### 4. Setting up to ensure contribution standards

Based on previous research, the Rust ecosystem lacks a very robust/mature and fully integrated solution for enforcing code contribution standards, such as conventional commits, pre-commit hooks, and automated linting checks. 

By leveraging non-rust packages like `Husky` and `Commitlint`, the project gains a comprehensive cross-language workflow that ensures:

- Standardized commit messages across all contributors

- Automatic pre-commit checks (formatting, linting, compilation)

- Pre-push checks enforcement

> The project however stays focused - with it's core as pure Rust. The `NodeJs` integration only introduced the packages needed for enforcing code/contribution standards on the project.
>
> P.S: The preferred `NodeJs` package manager is `Bun`.


To integrate the `Husky` and `Commitlint` setup into your current Rust workflow:

1. Ensure to sync with the main branch and pull in all updates:

```shell
git pull origin main
```

2. Install the new packages with Bun and ensure Husky is properly initialized:

```shell
bun install && husky init
```

## Project Config Setup

The project uses a highly flexible configuration pattern powered by the `config` crate.

### Loading Order(Arranged in increasing order of overriding authority):

1. **Base Config**: `config/base.toml` (Default values).

2. **Environment Config Overrides**: `config/{APP__ENV}.toml` (e.g., `development.toml`, `production.toml`).

3. **Local Overrides**: `config/local.toml` 

4. **Environment Variables**: Prefixed with `APP__`.

### Mapping Rule for Environment Variables

`__`(double underscore) is used as a separator to map to nested TOML sections.

**Syntax:** `APP__<SECTION>__<FIELD>=value`

**Example:**

To override the server port in `base.toml`:

```toml
[server]
port = 8000
```

Set the environment variable:

`APP__SERVER__PORT=9000`

### Mandatory Sections

The `validate()` method ensures the following sections are correctly populated at startup:

- `app`: Basic metadata.

- `server`: Host, Port, and Request Timeouts.

- `database`: Engine, Connection Pool settings, and Auth.

- `auth`: JWT Secret and Expiration lifetimes.

## Environment Variables Files

The project uses several `.env` files to manage environment-specific configurations. 

- `.env`: The default environment file containing shared variables or the primary `APP__ENV` setting.

- `.env.development`: Contains configuration overrides specifically for local development.

- `.env.staging`: Configured for the staging/pre-production environment.

- `.env.production`: Contains sensitive production-only credentials and settings.

> **Note:** Do not share real `.env` files (especially production). Use a sanitized `.env.example` for required keys and retrieve real secrets through the approved secret-management process.

## Testing

The project maintains high reliability through two layers of testing.

### 1. Unit Tests

Located within the source files (e.g., `src/utils/generate_tokens.rs`). They test isolated logic like hashing, token generation, and config validation.

**Run unit tests:**

```shell
cargo test --lib
```

### 2. Integration Tests

Located in the `tests` directory. They spin up a real server instance and a test database to verify end-to-end API flows.

**Available Integration Tests:**

- `login_test.rs`: Successful login, invalid credentials, non-existent users.

- `register_test.rs`: New user creation, duplicate email/phone prevention.

- `logout_test.rs`: Token invalidation and cookie clearing.

**Run integration tests:**

```shell
cargo test --test '*'
```

### 3. How to add new tests

- **Unit Tests**: Add a `#[cfg(test)] mod tests { ... }` block at the end of your module.

- **Integration Tests**: 

  1. Create a new file in `tests`.

  2. Use `common::setup_test_server().await` to get a `TestServer` instance.

  3. Use the request/response structs defined in `tests/common/mod.rs` for type-safe interaction.

## Reliability & Robustness

- **Request Timeouts**: Configurable via `server.request_timeout_secs`.

- **Database Pooling**: Managed via `PgPoolOptions` with configurable `max_connections`.

- **Environment-Aware Cookies**: Cookies are automatically set to `Secure` in production and `Lax`/`Insecure` (for HTTP) in development.

## Logging Implementation Layers

The application implements logging through multiple layers to ensure full visibility:

1. **Central Logging Middleware**: A top-level middleware that captures every incoming request and outgoing response, logging metadata such as HTTP method, path, status codes, and processing time.

> This prevents the need to manually register logs if no errors/issues are encountered on a request.

2. **In-Process Logic Logs**: Granular logs emitted directly from within the application logic (e.g., during errors) using the `tracing` crate to capture specific runtime context.

## Operating System Notes (WSL)

If you are developing on **WSL**, file system events might not trigger `cargo watch`. The project's `cargo dev` alias is pre-configured to use `--poll` if needed. 

Ensure your `APP__ENV`(in `.env`) is set to `development` to pick up the correct local database settings.

Cheers!!! 🍻
