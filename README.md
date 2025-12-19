# Rust Backend Web Server:

- Using Axum for the server and diesel for ORM

## Setup:

### 1. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Diesel CLI (Postgres)

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### 3. Create a .env file & Copy template:

```bash
cp .env.sample .env
```

### 4. Diesel Setup & Create migrations:

```bash
diesel setup
diesel migration generate create_users
diesel migration run # Run Migrations
```

### 5. Run the Server:

```bash
cd src
cargo run # Runs the webserver at http://localhost:3000
```

## Project Structure

```txt
├── .github/              # GitHub workflows & configs
├── migrations/           # Diesel database migrations
├── src/
│   ├── controllers/      # Request handlers / business logic
│   ├── db/               # Database connection & schema
│   ├── middlewares/      # Custom middlewares
│   ├── routes/           # Route definitions
│   └── main.rs           # Application entry point
├── tests/                # Integration & unit tests
├── .env                  # Environment variables
├── .env.sample           # Sample env config
├── Cargo.toml            # Dependencies & project metadata
├── Cargo.lock
├── diesel.toml           # Diesel configuration
├── Dockerfile
├── README.md
└── NOTES.md
```
