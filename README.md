# Warp/Diesel/JWT Rust API

## Crates

- bcrypt
- diesel
- jsonwebtoken
- serde
- warp
- and others...

## Database configuration

As pointed in the [official documentation](https://diesel.rs/guides/getting-started), first install the Diesel CLI :

```bash
# This app is meant to work with a PostgreSQL db
cargo install diesel_cli --no-default-features --features postgres
```

Then copy the `.env.sample` file into a `.env` and adapt it to your needs :

```bash
cp .env.sample .env
```

Finally, create the database and run migrations with `setup` :

```bash
diesel setup
```

## Endpoints

| Route | Method | Data |
|---|---|---|
| /users | POST | { "email": email, "password": password } |
| /users | GET | Authorization header : Bearer Token fetched from login |
| /login | POST | { "email": email, "password": password } |

## Run the project

```bash
# Default API port is 3500
cargo run
```
