# Contributing to Sultan Server

Thank you for considering contributing to Sultan Server! This guide will help you get started.

## Prerequisites

- Rust 1.75 or later
- SQLite (for local development)
- PostgreSQL (optional, for testing)
- Docker (optional, for integration tests)

## Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sultanpos/serverust.git
   cd serverust
   ```

2. **Set up environment:**
   ```bash
   cp .env.sqlite.example .env
   # or for PostgreSQL:
   # cp .env.postgres.example .env
   ```

3. **Install dependencies:**
   ```bash
   cargo build
   ```

4. **Run migrations:**
   ```bash
   # Migrations run automatically on startup
   cargo run
   ```

## Testing

### Local Testing

```bash
# Run all tests with SQLite
cargo test

# Test with PostgreSQL (update .env first)
./switch-db.sh postgres
cargo test

# Switch back to SQLite
./switch-db.sh sqlite
cargo test
```

### Running Specific Tests

```bash
# Run tests in a specific module
cargo test user_service

# Run tests with output
cargo test -- --nocapture

# Run tests with specific name pattern
cargo test test_create_user
```

## Code Quality

### Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Auto-format code
cargo fmt --all
```

### Linting

```bash
# Run Clippy
cargo clippy --all-targets --all-features

# Fix Clippy warnings automatically
cargo clippy --fix
```

### Security Audit

```bash
# Install cargo-audit
cargo install cargo-audit

# Run security audit
cargo audit
```

## CI/CD Pipeline

Our GitHub Actions pipeline runs automatically on:
- Every push to `main`, `dev`, or `develop` branches
- Every pull request to these branches

### Pipeline Jobs

1. **Test Matrix:**
   - Runs tests with SQLite
   - Runs tests with PostgreSQL
   - Executes on Ubuntu latest

2. **Code Quality Checks:**
   - Formatting (`cargo fmt`)
   - Linting (`cargo clippy`)
   - Security audit (`cargo audit`)

3. **Code Coverage:**
   - Generates coverage reports
   - Uploads to Codecov (if configured)

## Pull Request Guidelines

### Before Creating a PR

Ensure your code passes all checks:

```bash
# 1. Format code
cargo fmt --all

# 2. Run Clippy
cargo clippy --all-targets --all-features

# 3. Run tests
cargo test

# 4. Test both databases
./switch-db.sh sqlite && cargo test
./switch-db.sh postgres && cargo test
```

### PR Requirements

- [ ] Code is properly formatted (`cargo fmt`)
- [ ] No Clippy warnings
- [ ] All tests pass
- [ ] PR title follows conventional commits format
- [ ] Changes are documented (if needed)
- [ ] New tests added for new features

### PR Title Format

Use **Conventional Commits** format:

```
<type>(<scope>): <description>

Examples:
- feat: add user authentication
- fix(db): resolve connection pool issue
- docs: update API documentation
- refactor(auth): simplify token validation
- test: add integration tests for user service
- chore: update dependencies
```

**Types:**
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation updates
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Test additions/updates
- `chore:` - Maintenance tasks (dependencies, configs)

**Optional scope:**
- `auth` - Authentication/authorization
- `db` - Database related
- `api` - API endpoints
- `config` - Configuration
- `deps` - Dependencies

## Database Development

### Adding New Migrations

Create migrations for both databases:

**SQLite:**
```bash
# Create new migration
sqlx migrate add -r create_new_table --source migrations-sqlite

# Edit the generated files:
# - migrations-sqlite/XXXXXX_create_new_table.up.sql
# - migrations-sqlite/XXXXXX_create_new_table.down.sql
```

**PostgreSQL:**
```bash
# Create new migration
sqlx migrate add -r create_new_table --source migrations

# Edit the generated files:
# - migrations/XXXXXX_create_new_table.up.sql
# - migrations/XXXXXX_create_new_table.down.sql
```

### Testing Migrations

```bash
# Test SQLite migration
sqlx migrate run --source migrations-sqlite --database-url sqlite:test.db

# Test PostgreSQL migration
sqlx migrate run --source migrations --database-url postgres://user:pass@localhost/testdb
```

## Project Structure

```
src/
├── application/         # Business logic layer
│   ├── app_error.rs    # Error handling
│   └── user_service.rs # User business logic
├── crypto/             # Cryptographic operations
│   └── password.rs     # Password hashing
├── domain/             # Domain models
│   └── user.rs        # User entity
├── persistence/        # Data access layer
│   ├── user_repo.rs   # Repository trait
│   ├── postgres/      # PostgreSQL implementations
│   │   └── user.rs
│   └── sqlite/        # SQLite implementations
│       └── user.rs
├── web/               # HTTP layer
│   ├── user_routes.rs # User endpoints
│   └── app_state.rs   # Application state
├── config.rs          # Configuration management
├── server.rs          # Server initialization
└── main.rs           # Application entry point
```

## Adding New Repository Implementations

To add support for a new database (e.g., MongoDB):

1. **Create directory structure:**
   ```bash
   mkdir -p src/persistence/mongodb
   ```

2. **Create implementation:**
   ```rust
   // src/persistence/mongodb/user.rs
   use crate::persistence::user_repo::UserRepository;
   
   pub struct MongoUserRepository {
       // implementation
   }
   
   #[async_trait]
   impl UserRepository for MongoUserRepository {
       // implement trait methods
   }
   ```

3. **Update mod.rs:**
   ```rust
   // src/persistence/mod.rs
   pub mod mongodb;
   pub use mongodb::MongoUserRepository;
   ```

4. **Update DbPool enum:**
   ```rust
   pub enum DbPool {
       Postgres(PgPool),
       Sqlite(SqlitePool),
       Mongo(mongodb::Client),
   }
   ```

## Docker Testing

### Build and Test

```bash
# Build Docker image
docker build -t sultan-server .

# Run with SQLite
docker run -e DATABASE_TYPE=sqlite -e DATABASE_URL=sqlite:./sultan.db sultan-server

# Run with PostgreSQL (requires postgres container)
docker-compose up
```

### Integration Tests with Docker

```bash
# Run integration tests
docker-compose -f docker-compose.test.yml up --build --abort-on-container-exit

# Clean up
docker-compose -f docker-compose.test.yml down
```

## Getting Help

- **Issues:** Check [existing issues](https://github.com/sultanpos/serverust/issues) or create a new one
- **Discussions:** Use [GitHub Discussions](https://github.com/sultanpos/serverust/discussions) for questions
- **Security:** Report security issues privately to apin.klas@gmail.com

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## License

By contributing to Sultan Server, you agree that your contributions will be licensed under the GPL-3.0 license.

Thank you for contributing! 🚀
