# Squirrel

A local credit card bill analysis tool to squirrel away you money

## Initialize database

Set `DATABASE_URL` in `root/.env` to your sqlite database. The database file must exist but can be empty.

Run from root.

```bash
sea-orm-cli migrate fresh -d ./crates/migration
```

## Generate SeaOrm Entities

Run from root.

Creates rust models from the database and puts them in a module at output

```bash
sea-orm-cli generate entity -o ./crates/db-entity/src/entity
```
