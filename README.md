# dicetower

## Development

1. Install [https://www.rust-lang.org/tools/install](rustup) and use it to install Rust to your system
1. Run postgres, ie through docker `docker run --name postgres -e POSTGRES_PASSWORD=${password} -p 5432:5432 -d postgres:16.4-alpine`
1. `echo "DATABASE_URL=postgres://postgres:${password}@localhost/dicetower" >> .env`
1. Install [https://diesel.rs/guides/getting-started](diesel-cli) and run migrations `diesel migration run`
1. `cargo run`
