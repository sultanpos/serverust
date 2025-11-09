# Database Configuration Guide

This application supports both PostgreSQL and SQLite databases. You can choose which database to use via the `.env` file configuration.

## Configuration

The database type is controlled by the `DATABASE_TYPE` environment variable in your `.env` file.

### SQLite Configuration (Default)

SQLite is the default database and requires no additional setup. It's perfect for development and small deployments.

```env
DATABASE_TYPE=sqlite
DATABASE_URL=sqlite://./sultan.db
```

You can also use an in-memory database for testing:
```env
DATABASE_TYPE=sqlite
DATABASE_URL=sqlite::memory:
```

### PostgreSQL Configuration

For production deployments or when you need advanced database features, use PostgreSQL:

```env
DATABASE_TYPE=postgres
DATABASE_URL=postgres://username:password@localhost/database_name
```

Replace `username`, `password`, and `database_name` with your PostgreSQL credentials.

## Migrations

The application automatically runs migrations on startup based on the configured database type:

- **PostgreSQL**: Migrations from `./migrations/` directory
- **SQLite**: Migrations from `./migrations-sqlite/` directory

Both migration sets maintain the same schema structure, adapted for each database's specific syntax.

## Running Migrations Manually

### SQLite

```bash
sqlx migrate run --database-url sqlite://./sultan.db --source ./migrations-sqlite
```

### PostgreSQL

```bash
sqlx migrate run --database-url postgres://user:password@localhost/dbname --source ./migrations
```

## Example Configurations

See the example files:
- `.env.sqlite.example` - SQLite configuration example
- `.env.postgres.example` - PostgreSQL configuration example

Copy one of these to `.env` and modify as needed:

```bash
# For SQLite
cp .env.sqlite.example .env

# For PostgreSQL
cp .env.postgres.example .env
# Then edit .env with your PostgreSQL credentials
```

## Features

- **Automatic database creation**: SQLite databases are created automatically if they don't exist
- **Automatic migrations**: Migrations run on application startup
- **Seamless switching**: Change database type by updating `.env` and restarting
- **Type safety**: Both implementations use the same repository interface
