# Krabby/chat/auth_server.

AUTH REST API layer/service for the Krabby `chat` implementation.

## 🚀 Features

- **JWT Authentication**: Secure access and refresh tokens.
- **Argon2 Hashing**: Industry-standard password hashing.
- **PostgreSQL Persistence**: Reliable data storage using `sqlx`.
- **Dynamic Configuration**: Multi-layer configuration system (TOML + Environment Variables).
- **Robust Error Handling**: Domain-specific error types and clear API responses.
- **Comprehensive Testing**: Unit tests for utilities and integration tests for API endpoints.
- **Middleware Integration**: Logging, Request Timeouts, and Cookie management.

---

## 🛠️ Setup & Execution

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Docker](https://www.docker.com/) (for database)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)

### 2. Database Setup
Start the local PostgreSQL database:

```shell
docker run -d --name rusty-chat__dev_db -p 5433:5432 -e POSTGRES_USER=okpainmo -e POSTGRES_PASSWORD=supersecret -e POSTGRES_DB=rusty_chat_db_dev postgres
```

Initialize the schema:
```shell
sqlx migrate run --database-url postgres://okpainmo:supersecret@localhost:5433/rusty_chat_db_dev
```

### 3. Running the Server
The project uses `cargo-watch` for an improved development experience.

**Development mode (with auto-reload):**
```shell
cargo dev
```
*Note: This is an alias for `cargo watch`. If you are on WSL and reload doesn't trigger, it uses polling (configured in `.cargo/config.toml`).*

**Standard run:**
```shell
cargo run
```

---

## ⚙️ Configuration System

The project uses a highly flexible configuration pattern powered by the `config` crate.

### Loading Order (Highest Priority First):
1. **Environment Variables**: Prefixed with `APP__`.
2. **Local Overrides**: `config/local.toml` (Git-ignored, for machine-specific settings).
3. **Environment Overrides**: `config/{APP__ENV}.toml` (e.g., `development.toml`, `production.toml`).
4. **Base Config**: `config/base.toml` (Default values).

### Mapping Rule for Environment Variables
Use `__` (double underscore) as a separator to map to nested TOML sections.

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

---

## 🧪 Testing

The project maintains high reliability through two layers of testing.

### 1. Unit Tests
Located within the source files (e.g., `src/utils/generate_tokens.rs`). They test isolated logic like hashing, token generation, and config validation.

**Run unit tests:**
```shell
cargo test --lib
```

### 2. Integration Tests
Located in the `tests/` directory. They spin up a real server instance and a test database to verify end-to-end API flows.

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
  1. Create a new file in `tests/`.
  2. Use `common::setup_test_server().await` to get a `TestServer` instance.
  3. Use the request/response structs defined in `tests/common/mod.rs` for type-safe interaction.

---

## 🛡️ Reliability & Robustness
- **Request Timeouts**: Configurable via `server.request_timeout_secs`.
- **Database Pooling**: Managed via `PgPoolOptions` with configurable `max_connections`.
- **Environment-Aware Cookies**: Cookies are automatically set to `Secure` in production and `Lax`/`Insecure` (for HTTP) in development.

---

## 🐧 Operating System Notes (WSL)
If you are developing on **WSL**, file system events might not trigger `cargo watch`. The project's `cargo dev` alias is pre-configured to use `--poll` if needed. Ensure your `APP__ENV` is set to `development` to pick up the correct local database settings.

Cheers!!! 🍻
