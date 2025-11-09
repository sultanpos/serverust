# GitHub Actions Quick Reference

## 📋 Quick Commands

### Local Testing
```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run lints
cargo clippy --all-targets --all-features

# Fix clippy warnings
cargo clippy --fix

# Run tests
cargo test

# Test with SQLite
./switch-db.sh sqlite && cargo test

# Test with PostgreSQL  
./switch-db.sh postgres && cargo test

# Security audit
cargo audit
```

### Before Committing
```bash
# One-liner to check everything
cargo fmt --all && \
cargo clippy --all-targets --all-features && \
cargo test
```

## 🏷️ PR Title Format

```
<type>(<scope>): <description>

Types:
  feat:     New feature
  fix:      Bug fix
  docs:     Documentation
  style:    Formatting
  refactor: Code restructuring
  test:     Adding tests
  chore:    Maintenance

Examples:
  feat: add user authentication
  fix(db): resolve connection pool issue
  docs: update API documentation
```

## ✅ CI Workflow Jobs

| Job | Purpose | Duration |
|-----|---------|----------|
| Test (SQLite) | Run tests with SQLite | ~1-2 min |
| Test (PostgreSQL) | Run tests with PostgreSQL | ~1-2 min |
| Security Audit | Check vulnerabilities | ~30 sec |
| Code Coverage | Generate coverage report | ~2-3 min |

## 🔍 PR Checks

| Check | What it does | How to fix locally |
|-------|--------------|-------------------|
| PR Title | Validates conventional commits | Update PR title |
| Code Formatting | Checks `cargo fmt` | Run `cargo fmt --all` |
| Clippy Lints | Checks for code issues | Run `cargo clippy --fix` |
| Tests | Runs all tests | Run `cargo test` |

## 🚨 Common Issues

### ❌ "Code is not properly formatted"
```bash
cargo fmt --all
git add .
git commit --amend --no-edit
git push --force
```

### ❌ "Clippy found issues"
```bash
cargo clippy --all-targets --all-features -- -D warnings
# Fix the issues shown
git add .
git commit -m "fix: address clippy warnings"
git push
```

### ❌ "Tests failed"
```bash
# Run tests with same environment as CI
export DATABASE_TYPE=sqlite
export DATABASE_URL=sqlite:test.db
cargo test

# Check specific test
cargo test test_name -- --nocapture
```

### ❌ "PR title format invalid"
```bash
# Update PR title on GitHub to follow format:
# feat: description
# fix: description
# docs: description
```

## 📊 Viewing Results

1. **Go to Actions tab** on GitHub
2. **Click on workflow run** to see details
3. **Click on job** to see logs
4. **Click on step** to see detailed output

## 🔄 Re-running Failed Jobs

1. Go to failed workflow run
2. Click "Re-run jobs" button
3. Select "Re-run failed jobs" or "Re-run all jobs"

## 📦 Files Structure

```
.github/
├── workflows/
│   ├── ci.yml              # Main CI workflow
│   └── pr-checks.yml       # PR validation
├── ISSUE_TEMPLATE/
│   ├── bug_report.md       # Bug template
│   └── feature_request.md  # Feature template
├── PULL_REQUEST_TEMPLATE.md
└── WORKFLOWS.md            # Full documentation

Dockerfile                   # Container build
docker-compose.test.yml     # Integration tests
.dockerignore               # Docker optimization
CONTRIBUTING.md             # Contribution guide
```

## 🎯 Branch Protection Settings

**Recommended settings for `main` and `dev` branches:**

- ✅ Require pull request reviews (1 approval)
- ✅ Require status checks to pass:
  - Test (SQLite)
  - Test (PostgreSQL)
  - Code Validation
  - Security Audit
- ✅ Require branches to be up to date
- ✅ Require conversation resolution
- ⚠️ Do not allow force push
- ⚠️ Do not allow deletions

## 💡 Pro Tips

1. **Use Draft PRs** for work in progress
2. **Add tests** for all new features
3. **Keep PRs small** and focused
4. **Write descriptive commit messages**
5. **Test locally** before pushing
6. **Review your own PR** before requesting reviews
7. **Use conventional commits** for all commits
8. **Update documentation** with code changes

## 🔗 Quick Links

- [Full Documentation](.github/WORKFLOWS.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Setup Summary](../GITHUB_ACTIONS_SETUP.md)
