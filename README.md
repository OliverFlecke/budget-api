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
  - Note: Currently this is a very simple "authorization" that simply allow you to specify your user id. There is NO actual security in this
  - [ ] JWT (or similar) authorization

## Build

Application is tested with Rust v1.64, but newer will likely work.
Start the application with:

```sh
cargo run
```

## Database

It requires a Postgres database to run against and persist data.
See `scripts/start_db.sh` to start Postgres in a Docker container.
The default connection string is stored in `.env` (the format is specified by `sqlx` to allow it to statically compile queries).
The connection string must be provided as the environment variable `DATABASE_URL`, which can be done with:

```sh
export $(cat .env)
```

Migrations are handled through [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).
All migrations are stored as revertable SQL files in the `migrations` directory.
After installing the tool (see previous link), these can be run with `sqlx migrate run` or `sqlx migrate revert`.

If it is the first time running the application, use `sql database create` to create a database inside of Postgres.
