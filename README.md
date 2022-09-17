# credit-card-analysis

## Initialize database

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
