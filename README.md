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
cargo install diesel_cli
```

Then copy the `.env.sample` file into a `.env` and adapt it to your needs :

```bash
cp .env.sample .env
```

Finally, create the database and run migrations with `setup` :

```bash
diesel setup
```
