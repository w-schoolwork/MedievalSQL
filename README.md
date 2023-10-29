# MedievalSQL

A toy web service for organizing "tournaments", made as a project for CS 425, Database Organization.

## Building

MedievalSQL is pure Rust, so Cargo and the Rust compiler are the only hard compile-time dependencies.

However, we use [sqlx](https://docs.rs/sqlx) to type-check SQL queries at compile-time, so changing the database abstractions will require Postgres to be accessible at compile-time, and you need to have the `sqlx-cli` crate installed to rebuild the type information in the `.sqlx` directory.

Build process:

* If you have changed a migration:
  * Run `./recreate-db.sh`
  * This is a destructive operation.
* If you have changed the database abstractions:
  * Run `cargo sqlx prepare`
* Run `cargo build`
