# GitHub Actions CI/CD Documentation

This document describes the CI/CD pipeline configuration for Sultan Server.

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main`, `dev`, or `develop` branches
- Pull requests to these branches

**Jobs:**

#### Test Matrix
Runs tests against both SQLite and PostgreSQL databases:

```yaml
strategy:
  matrix:
    database: [sqlite, postgres]
```

**Steps:**
1. Checkout code
2. Install Rust toolchain with `rustfmt` and `clippy`
3. Cache Cargo dependencies
4. Install `sqlx-cli`
5. Setup test environment (database-specific)
6. Run migrations
7. Check code formatting
8. Run Clippy lints
9. Build the project
10. Run all tests

#### Security Audit
- Runs `cargo audit` to check for security vulnerabilities
- Uses `cargo-audit` tool

#### Code Coverage
- Generates code coverage reports using `cargo-llvm-cov`
- Tests with SQLite database
- Uploads coverage to Codecov (optional)

### 2. PR Checks Workflow (`.github/workflows/pr-checks.yml`)

**Triggers:**
- Pull request opened, synchronized, or reopened

**Jobs:**

#### PR Validation
- Validates PR title follows conventional commits format
- Checks for breaking changes
- Automatically comments on new PRs with helpful information

#### Code Validation
- Checks code formatting with `cargo fmt`
- Runs Clippy lints
- Searches for TODO/FIXME comments (warning only)

## Environment Variables

All workflows use these environment variables:

```yaml
env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
```

## Services

### PostgreSQL Service
Used in the CI workflow for PostgreSQL testing:

```yaml
services:
  postgres:
    image: postgres:15
    env:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: sultan_test
    ports:
      - 5432:5432
    options: >-
      --health-cmd pg_isready
      --health-interval 10s
      --health-timeout 5s
      --health-retries 5
```

## Caching Strategy

Cargo dependencies are cached to speed up builds:

```yaml
- name: Cache Cargo dependencies
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

## Test Environment Setup

### SQLite
```bash
DATABASE_TYPE=sqlite
DATABASE_URL=sqlite:test.db
RUST_LOG=sultan=debug,tower_http=debug,sqlx=warn
REFRESH_TOKEN_TTL_DAYS=30
JWT_SECRET=test_secret_for_ci
```

### PostgreSQL
```bash
DATABASE_TYPE=postgres
DATABASE_URL=postgres://postgres:postgres@localhost:5432/sultan_test
RUST_LOG=sultan=debug,tower_http=debug,sqlx=warn
REFRESH_TOKEN_TTL_DAYS=30
JWT_SECRET=test_secret_for_ci
```

## Badge Status

Add these badges to your README.md:

```markdown
![CI](https://github.com/sultanpos/serverust/workflows/CI/badge.svg)
![PR Checks](https://github.com/sultanpos/serverust/workflows/PR%20Checks/badge.svg)
[![codecov](https://codecov.io/gh/sultanpos/serverust/branch/main/graph/badge.svg)](https://codecov.io/gh/sultanpos/serverust)
```

## Failure Troubleshooting

### Common Issues

1. **Formatting Failures**
   ```bash
   # Fix locally
   cargo fmt --all
   git commit -am "fix: format code"
   ```

2. **Clippy Warnings**
   ```bash
   # Check warnings
   cargo clippy --all-targets --all-features
   
   # Fix automatically where possible
   cargo clippy --fix
   ```

3. **Test Failures**
   ```bash
   # Run tests locally with same environment
   export DATABASE_TYPE=sqlite
   export DATABASE_URL=sqlite:test.db
   cargo test
   ```

4. **Migration Failures**
   ```bash
   # Check migrations syntax
   sqlx migrate run --source migrations-sqlite --database-url sqlite:test.db
   sqlx migrate run --source migrations --database-url postgres://user:pass@localhost/testdb
   ```

## Performance Optimization

### Build Times

Current optimizations:
- ✅ Cargo dependency caching
- ✅ Incremental compilation
- ✅ Parallel test execution

Typical build times:
- Cold build: ~5-7 minutes
- Cached build: ~1-2 minutes
- Tests: ~30-60 seconds per database

### Future Improvements

- [ ] Use `sccache` for compiler caching
- [ ] Implement build artifacts caching
- [ ] Parallel database tests
- [ ] Custom Docker images with pre-installed dependencies

## Security Considerations

### Secrets Management

Never commit sensitive data. Use GitHub Secrets for:
- Database passwords
- API keys
- JWT secrets
- Service credentials

### Audit Schedule

- `cargo audit` runs on every CI build
- Weekly scheduled audits recommended
- Dependabot enabled for automatic updates

## Local Workflow Testing

### Using `act` Tool

You can test GitHub Actions locally using [act](https://github.com/nektos/act):

```bash
# Install act
# macOS
brew install act

# Linux
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run workflows locally
act push
act pull_request
```

### Docker-based Testing

```bash
# Run integration tests locally
docker-compose -f docker-compose.test.yml up --build

# Clean up
docker-compose -f docker-compose.test.yml down
```

## Monitoring and Notifications

### GitHub Notifications

Configure repository notifications:
1. Go to repository Settings
2. Navigate to Notifications
3. Enable desired notification types

### Status Checks

Required status checks (recommended):
- ✅ Test (SQLite)
- ✅ Test (PostgreSQL)
- ✅ Code Validation
- ✅ Security Audit

Configure in: Settings → Branches → Branch protection rules

## Continuous Deployment

### Deployment Workflow (Future)

To add automatic deployment:

```yaml
name: Deploy

on:
  push:
    branches: [ main ]
    tags:
      - 'v*'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t sultan-server:${{ github.sha }} .
      - name: Push to registry
        # Add deployment steps
```

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust GitHub Actions](https://github.com/actions-rs)
- [sqlx Documentation](https://github.com/launchbadge/sqlx)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
