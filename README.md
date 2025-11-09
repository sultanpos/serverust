# Sultan Server

A high-performance Sultan POS server with multi-database support.

## Features

- 🚀 Built with Axum web framework
- 🔐 User authentication with Argon2 password hashing
- 💾 **Multi-Database Support**: PostgreSQL and SQLite
- 🔄 Automatic database migrations
- 📝 Structured logging (console + JSON file)
- 🏗️ Clean Architecture design pattern
- ⚡ Async/await with Tokio runtime

## Database Support

This application supports both PostgreSQL and SQLite databases. Choose your database by setting the `DATABASE_TYPE` in the `.env` file:

- **SQLite** (Default): Perfect for development and small deployments
- **PostgreSQL**: Recommended for production use

### Quick Start with SQLite

```bash
# Copy the SQLite example configuration
cp .env.sqlite.example .env

# Run the application
cargo run
```

The SQLite database will be created automatically at `./sultan.db`.

### Quick Start with PostgreSQL

```bash
# Copy the PostgreSQL example configuration
cp .env.postgres.example .env

# Edit .env with your PostgreSQL credentials
nano .env

# Run the application
cargo run
```

### Databases

For detailed database configuration, see [DATABASE.md](DATABASE.md).

## Installation

### Prerequisites

- Rust 1.70+ (or latest stable)
- For PostgreSQL: PostgreSQL 12+ server
- For SQLite: No additional setup needed

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

The server will start on `http://127.0.0.1:3001`.

## Configuration

Configuration is managed through environment variables in the `.env` file:

```env
DATABASE_TYPE=sqlite                    # Database type: sqlite or postgres
DATABASE_URL=sqlite://./sultan.db       # Database connection string
RUST_LOG="sultan=debug,tower_http=debug,sqlx=warn"
REFRESH_TOKEN_TTL_DAYS="30"
JWT_SECRET=replace_this_with_a_random_secret
```

See example configuration files:
- `.env.sqlite.example` - SQLite configuration
- `.env.postgres.example` - PostgreSQL configuration

## Project Structure

```
src/
├── application/     # Business logic layer
│   ├── user_service.rs
│   └── app_error.rs
├── crypto/          # Cryptographic operations
│   └── password.rs
├── domain/          # Domain models
│   └── user.rs
├── persistence/     # Database layer
│   └── user_repo.rs  # Multi-database repository
├── web/             # HTTP layer
│   ├── user_routes.rs
│   └── app_state.rs
├── config.rs        # Configuration management
├── server.rs        # Server initialization
└── main.rs          # Application entry point

migrations/          # PostgreSQL migrations
migrations-sqlite/   # SQLite migrations
```

## Development

### Running Tests

```bash
cargo test
```

### Database Migrations

Migrations run automatically on application startup. To run migrations manually:

```bash
# SQLite
sqlx migrate run --database-url sqlite://./sultan.db --source ./migrations-sqlite

# PostgreSQL
sqlx migrate run --database-url postgres://user:password@localhost/dbname --source ./migrations
```

## License

GPL-3.0

## Author

Apin <apin.klas@gmail.com>
