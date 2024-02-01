# Learning Actix-web

## Environment Variables

The file `.env` is included in the repository for learning purposes.

## SQLX

### Migrate the database

```shell
# Set the database connection
DATABASE_URL=postgresql://dev:metallica@localhost:5432/actixwebapi

# Run the migrate command
sqlx migrate run
```
