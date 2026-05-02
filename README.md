# RustGym

An interactive, gamified platform for learning Rust programming. RustGym features a quest-based curriculum with 500+ coding challenges, Docker-isolated code execution, gamification mechanics (XP, levels, streaks, achievements, leaderboards), community features, and a CLI tool for local development.

## Architecture

| Service        | Technology                   | Port |
| -------------- | ---------------------------- | ---- |
| Frontend       | React 18 + TypeScript + Vite | 5173 |
| Backend API    | Rust / Axum                  | 3000 |
| Runner Service | Rust / Axum + Docker         | 3001 |
| Database       | PostgreSQL 16                | 5432 |
| Cache          | Redis 7                      | 6379 |
| Object Storage | MinIO (S3-compatible)        | 9000 |

## Project Structure

```
rustgym/
├── frontend/          # React SPA (Vite + TypeScript)
├── backend/           # Rust API server (Axum + SeaORM)
├── runner/            # Code execution microservice
├── cli/               # CLI tool for local challenge workflow
├── challenges/        # Challenge content (manifest + files)
├── docker-compose.yml # Local development infrastructure
└── .env.example       # Environment variable template
```

## Prerequisites

- [Node.js](https://nodejs.org/) 20+ and [pnpm](https://pnpm.io/) 9+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- Git

## Getting Started

### 1. Clone the repository

```bash
git clone <repository-url>
cd rustgym
```

### 2. Set up environment variables

```bash
cp .env.example .env
# Edit .env with your local configuration
```

### 3. Start infrastructure services

```bash
docker-compose up -d
```

This starts PostgreSQL, Redis, MinIO, and the runner service.

### 4. Run database migrations

```bash
cd backend
cargo run --bin migration
```

### 5. Seed challenge content

```bash
cd backend
cargo run --bin seeder
```

### 6. Start the backend API

```bash
cd backend
cargo run
```

The API will be available at `http://localhost:3000`.

### 7. Start the frontend

```bash
cd frontend
pnpm install
pnpm dev
```

The frontend will be available at `http://localhost:5173`.

## Development

### Running tests

```bash
# Frontend
cd frontend && pnpm test

# Backend
cd backend && cargo test

# Runner
cd runner && cargo test
```

### Code quality

```bash
# Rust linting
cargo clippy -- -D warnings

# Rust formatting
cargo fmt --check

# Frontend linting
cd frontend && pnpm lint
```

## CLI Tool

The `rustgym` CLI allows you to work on challenges locally:

```bash
cd cli && cargo install --path .

rustgym login
rustgym list --quest "rust-foundations"
rustgym download hello-ferris
rustgym submit hello-ferris
```

## License

All rights reserved.
