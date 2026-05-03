# 🦀 RustGym

An interactive, gamified platform for learning Rust programming. RustGym features a quest-based curriculum with 500+ coding challenges, Docker-isolated code execution, gamification mechanics (XP, levels, streaks, achievements, leaderboards), community features, and a CLI tool for local development.

> **🚧 Development Status**: This project is actively in development and **not yet hosted**. The backend requires local setup (see below). Contributions, feedback, and discussions are welcome!

> **🌐 Frontend Preview**: The UI is deployed at [https://rustgym.vercel.app/](https://rustgym.vercel.app/) (frontend only — requires a local backend to be fully functional).

> **💬 Open for Contribution**: If you're interested in contributing, please open an issue or discussion. All skill levels welcome.

## Architecture

| Service        | Technology                   | Port   | Status |
| -------------- | ---------------------------- | ------ | ------ |
| Frontend       | React 19 + TypeScript + Vite | 5173   | ✅     |
| Backend API    | Rust / Axum 0.7              | 3000   | ✅     |
| Runner Service | Rust / Axum + Docker         | 3001   | ✅     |
| CLI Tool       | Rust / Ratatui TUI           | —      | ✅     |
| Database       | PostgreSQL 16                | 5432   | ✅     |
| Cache          | Redis 7                      | 6379   | ✅     |
| Object Storage | MinIO (S3-compatible)        | 9000   | ✅     |
| Reverse Proxy  | Nginx                        | 80/443 | ✅     |

## What's Implemented

### ✅ Phase 0: Monorepo Bootstrap

- Vite + React 19 + TypeScript frontend with Tailwind CSS, shadcn/ui, Zustand, TanStack Query, React Router
- Rust/Axum backend with SeaORM, JWT auth, Argon2 password hashing
- Runner microservice with Docker executor
- Docker Compose for local infrastructure (Postgres, Redis, MinIO)
- GitHub Actions CI pipeline (lint, build, test for all services)

### ✅ Phase 1: Authentication & User System

- User registration with email/password (Argon2 hashing)
- JWT access tokens (15min) + refresh tokens (30-day, rotated)
- Login, logout, token refresh, email verification (stub), password reset
- Protected user profile routes (GET/PUT /users/me, GET /users/:username)
- JWT auth middleware with CurrentUser extractor
- Redis-backed rate limiting (20/min auth, 200/min global) with graceful degradation
- CORS configured for frontend origin

### ✅ Phase 2: Curriculum

- Database entities & migrations for quests, levels, tasks, progress tables
- Curriculum seeder binary with manifest.json (8 quests, 65 levels)
- API routes for browsing quests/levels/tasks with progress tracking

### ✅ Phase 3: Code Execution Engine

- Docker-sandboxed code execution with resource limits
- cargo test runner with output parsing
- Submission API with async execution and result storage

### ✅ Phase 4: Gamification

- XP system with level progression
- Daily streaks with freeze mechanics
- Achievement system (20+ achievements with unlock conditions)
- Global and quest-specific leaderboards

### ✅ Phase 5: Community

- Forum with posts and replies
- Article publishing system
- Peer code reviews

### ✅ Phase 6: Admin Panel

- Content management (quests, levels, tasks)
- User management and moderation

### ✅ Phase 7: CLI Tool & Deployment

- CLI tool for login, browsing, downloading, and submitting challenges
- Production Docker Compose with resource limits
- Nginx reverse proxy with TLS termination
- OpenAPI documentation at /api/docs

## Project Structure

```
rustgym/
├── frontend/          # React SPA (Vite + TypeScript + Tailwind)
│   ├── src/
│   │   ├── pages/         # Route-level components
│   │   ├── components/    # Reusable UI (layout, editor, quest, gamification)
│   │   ├── stores/        # Zustand state (auth, editor, ui)
│   │   ├── services/      # API client with interceptors
│   │   ├── hooks/         # Custom React hooks
│   │   ├── types/         # Shared TypeScript interfaces
│   │   └── lib/           # Utilities (cn, constants)
│   └── tailwind.config.js # Design tokens
├── backend/           # Rust API server (Axum + SeaORM)
│   ├── src/
│   │   ├── routes/        # HTTP handlers
│   │   ├── services/      # Business logic
│   │   ├── middleware/    # Auth, rate limiting, CORS
│   │   ├── dto/           # Request/response types
│   │   ├── config.rs      # Environment config
│   │   └── error.rs       # AppError with IntoResponse
│   ├── entity/            # SeaORM entities
│   └── migration/         # Database migrations
├── runner/            # Code execution microservice
│   ├── src/main.rs        # Axum server with /run/test and /run/playground
│   ├── Dockerfile         # Multi-stage build with Docker CLI
│   └── sandbox.Dockerfile # Isolated sandbox image
├── cli/               # Interactive TUI (Ratatui + crossterm)
│   ├── src/
│   │   ├── main.rs        # Entry point, terminal setup, event loop
│   │   ├── app.rs         # App state machine, screen dispatch
│   │   ├── action.rs      # Action enum (navigation, API, editor)
│   │   ├── event.rs       # Event stream (keyboard, tick, API responses)
│   │   ├── editor.rs      # Editor resolution and launch
│   │   ├── config.rs      # Config file (~/.rustgym/config.toml)
│   │   ├── api/           # HTTP client (auth, quests, tasks, submit)
│   │   ├── screens/       # Screen handlers (login, quest list, detail, submit)
│   │   └── widgets/       # Reusable TUI widgets (status bar, spinner, list)
│   └── tests/             # Property-based tests (proptest)
├── challenges/        # Curriculum content
│   └── manifest.json      # 8 quests, 65 levels definition
├── nginx/             # Reverse proxy configuration
│   └── nginx.conf         # TLS termination, gzip, rate limiting
├── .github/workflows/ # CI/CD pipelines
├── docker-compose.yml      # Local dev infrastructure
├── docker-compose.prod.yml # Production deployment
└── .env.example       # Environment variable template
```

## Prerequisites

- [Node.js](https://nodejs.org/) 20+ and [pnpm](https://pnpm.io/) 9+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- Git

## Getting Started

### Quick Start (recommended)

The `dev.sh` script handles everything — infrastructure, backend, runner, and frontend:

```bash
git clone <repository-url>
cd rustgym
cp .env.example .env

# Start all services with one command
bash dev.sh
```

This will:

1. Start Docker infrastructure (PostgreSQL, Redis, MinIO)
2. Wait for databases to be ready
3. Start the backend API (port 3000, auto-runs migrations)
4. Start the runner service (port 3001, code execution sandbox)
5. Start the frontend dev server (port 5173)

Once running, open `http://localhost:5173` to see the app.

### dev.sh Commands

```bash
bash dev.sh          # Start everything
bash dev.sh stop     # Stop all services
bash dev.sh seed     # Seed curriculum data (quests, levels, tasks)
bash dev.sh logs     # Tail backend logs
```

### Seed Curriculum Data

After first startup, seed the database with quests and challenges:

```bash
bash dev.sh seed
```

### Using the CLI

Once the backend is running locally:

```bash
cd cli
cargo run -- --api-url http://localhost:3000
```

### Manual Setup (alternative)

If you prefer to start services individually:

```bash
# 1. Start infrastructure
docker compose up -d postgres redis minio

# 2. Run the backend (auto-runs migrations)
cargo run --manifest-path backend/Cargo.toml

# 3. Seed curriculum data
cargo run --manifest-path backend/Cargo.toml --bin seed

# 4. Start the frontend
cd frontend && pnpm install && pnpm dev
```

Open `http://localhost:5173` — you'll see the landing page.

## API Endpoints

### Health

| Method | Path           | Description    |
| ------ | -------------- | -------------- |
| GET    | /api/v1/health | Service status |

### Auth (`/api/v1/auth`)

| Method | Path             | Description          |
| ------ | ---------------- | -------------------- |
| POST   | /register        | Create account       |
| POST   | /login           | Sign in              |
| POST   | /refresh         | Rotate tokens        |
| POST   | /logout          | Revoke refresh token |
| POST   | /verify-email    | Verify email (stub)  |
| POST   | /forgot-password | Request reset (stub) |
| POST   | /reset-password  | Reset password       |

### Users (`/api/v1/users`)

| Method | Path             | Description            |
| ------ | ---------------- | ---------------------- |
| GET    | /me              | Get own profile (auth) |
| PUT    | /me              | Update profile (auth)  |
| GET    | /:username       | Get public profile     |
| GET    | /me/achievements | User achievements      |

### Leaderboard (`/api/v1/users/leaderboard`)

| Method | Path | Description           |
| ------ | ---- | --------------------- |
| GET    | /    | Global XP leaderboard |

### Quests (`/api/v1/quests`)

| Method | Path   | Description       |
| ------ | ------ | ----------------- |
| GET    | /      | List all quests   |
| GET    | /:slug | Get quest details |

### Tasks (`/api/v1/tasks`)

| Method | Path   | Description             |
| ------ | ------ | ----------------------- |
| GET    | /      | List tasks (filterable) |
| GET    | /:slug | Get task details        |

### Submissions (`/api/v1/submissions`)

| Method | Path | Description           |
| ------ | ---- | --------------------- |
| POST   | /    | Submit code           |
| GET    | /    | List user submissions |
| GET    | /:id | Get submission result |

### Forum (`/api/v1/forum`)

| Method | Path               | Description      |
| ------ | ------------------ | ---------------- |
| GET    | /posts             | List forum posts |
| POST   | /posts             | Create post      |
| GET    | /posts/:id         | Get post         |
| POST   | /posts/:id/replies | Reply to post    |

### Articles (`/api/v1/articles`)

| Method | Path | Description    |
| ------ | ---- | -------------- |
| GET    | /    | List articles  |
| POST   | /    | Create article |
| GET    | /:id | Get article    |

### Reviews (`/api/v1/reviews`)

| Method | Path | Description   |
| ------ | ---- | ------------- |
| GET    | /    | List reviews  |
| POST   | /    | Create review |

### Admin (`/api/v1/admin`)

| Method | Path       | Description         |
| ------ | ---------- | ------------------- |
| GET    | /users     | List all users      |
| PUT    | /users/:id | Update user role    |
| GET    | /stats     | Platform statistics |

### API Documentation

| Path      | Description              |
| --------- | ------------------------ |
| /api/docs | Swagger UI (interactive) |

## CLI Tool

The RustGym CLI is a full-screen interactive TUI for solving challenges from your terminal. See the [CLI README](cli/README.md) for full documentation.

### Quick Start

```bash
cd cli
cargo install --path .
rustgym
```

### Highlights

- Vim-style navigation (j/k/Enter/b)
- Persistent login sessions
- Opens challenges in your editor (nvim/vim/vi/$EDITOR)
- Submit solutions and see results inline
- Progress tracking with XP display

Configuration is stored in `~/.rustgym/config.toml`.

## Production Deployment

### Prerequisites

- Docker and Docker Compose v2
- Domain with DNS pointing to your server
- TLS certificates (e.g., from Let's Encrypt)

### Setup

1. Clone the repository and configure environment:

```bash
git clone <repository-url>
cd rustgym
cp .env.example .env
# Edit .env with production values (strong JWT_SECRET, real POSTGRES_PASSWORD, etc.)
```

2. Place TLS certificates:

```bash
mkdir -p nginx/certs
cp /path/to/fullchain.pem nginx/certs/
cp /path/to/privkey.pem nginx/certs/
```

3. Build and start all services:

```bash
docker compose -f docker-compose.prod.yml up -d --build
```

4. Seed curriculum data:

```bash
docker compose -f docker-compose.prod.yml exec backend /app/seed
```

5. Verify deployment:

```bash
curl https://your-domain.com/api/v1/health
# {"status":"ok","service":"rustgym-backend"}
```

### Resource Limits

| Service  | Memory Limit | CPU Limit |
| -------- | ------------ | --------- |
| Backend  | 512 MB       | 1.0       |
| Frontend | 128 MB       | 0.5       |
| Runner   | 1 GB         | 2.0       |
| Nginx    | 128 MB       | 0.5       |
| Postgres | 512 MB       | 1.0       |
| Redis    | 256 MB       | 0.5       |

### Rate Limiting

- Global: 200 requests/minute per IP (Redis-backed sliding window)
- Auth routes: 20 requests/minute per IP
- Nginx layer: 200 req/min with burst of 50

## Development

```bash
# Frontend lint + build
cd frontend && pnpm lint && pnpm build

# Backend check + test
cd backend && cargo clippy -- -D warnings && cargo test

# Runner check
cd runner && cargo clippy -- -D warnings

# CLI check
cd cli && cargo clippy -- -D warnings
```

## Design System

- **Primary**: #CE422B (Rust orange-red)
- **Dark theme**: #0B0B0A → #242320 (4-level depth)
- **Fonts**: JetBrains Mono (code), Barlow Condensed (display), Barlow (body)
- **UI**: shadcn/ui primitives, Framer Motion animations, Lucide icons

## Tech Stack

| Layer    | Technologies                                                                                                                  |
| -------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Frontend | React 19, TypeScript, Vite, Tailwind CSS 3, shadcn/ui, Zustand, TanStack Query v5, React Router, Framer Motion, Monaco Editor |
| Backend  | Rust, Axum 0.7, SeaORM, Tokio, jsonwebtoken, Argon2, tower-http                                                               |
| Runner   | Rust, Axum, Bollard (Docker API), syn (AST parsing)                                                                           |
| CLI      | Rust, Ratatui, crossterm, Tokio, reqwest                                                                                      |
| Database | PostgreSQL 16, Redis 7, MinIO                                                                                                 |
| DevOps   | Docker Compose, GitHub Actions                                                                                                |

## License

All rights reserved.
