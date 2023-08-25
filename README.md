# Budget REST API in [Rust](https://rust-lang.org)

A simple API for creating and tracking a budget.
My main reason for building this project was to improve my Rust knowledge, learning how to build a simple CRUD application with [axum](https://crates.io/crates/axum) and [sqlx](https://crates.io/crates/sqlx).

The plan is to integrate as the backend for a budget app hosted at [finance.oliverflecke.me/budget](https://finance.oliverflecke.me/budget).

## Features

The API currently have the following features:

- [x] Create a **budget** linked to your user
- [x] Retrive a **budget**  with all its items
- [x] List all of a user's budgets
- [x] Add items with a **name**, **category**, and **amount** linked to the budget
- [x] Update items' **name**, **category**, or **amount**
- [x] Delete items from a budget
- [x] Authorize as a user
  - [x] JWT authorization

## Build

Application is tested with Rust v1.64, but newer will likely work.
Start the application with:

```sh
cargo run
```

## Testing

Test can be run with the stardard `cargo test`.

Note that not all tests are activated by default. As the database layer needs an active database to work (sqlx does not seem to support offline mode yet for tests), these are disabled by default. To run them, you need to have a running database (see below). All tests can then be run with `cargo test --features db_test`.

## Database

The easiest way to build/test/run the application is to set the environment variable `SQLX_OFFLINE=true`, which will use the precompiled queries to the database to compile.

It requires a Postgres database to run against and persist data.
See `scripts/start_db.sh` to start Postgres in a Docker container.
The default connection string **can** be stored in `.env` (the format is specified by `sqlx` to allow it to statically compile queries), which allows `sqlx` to statically verify queries.
The file should have the following content (modify if you change the details of the connection string):

```sh
DATABASE_URL=postgres://postgres:password@localhost:5432/finance
```

When actually running the server, the connection string must be provided as the environment variable `DATABASE_URL`, which can be done with:

```sh
export $(cat .env)
```

Migrations are handled through [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).
All migrations are stored as revertable SQL files in the `migrations` directory.
After installing the tool (see previous link), these can be run with `sqlx migrate run` or `sqlx migrate revert`.

If it is the first time running the application, use `sql database create` to create a database inside of Postgres.
