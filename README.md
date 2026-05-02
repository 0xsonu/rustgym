# 🦀 RustGym

An interactive, gamified platform for learning Rust programming. RustGym features a quest-based curriculum with 500+ coding challenges, Docker-isolated code execution, gamification mechanics (XP, levels, streaks, achievements, leaderboards), community features, and a CLI tool for local development.

> **Status**: Active development — Phase 1 (Auth & User System) complete, Phase 2 (Curriculum) in progress.

## Architecture

| Service        | Technology                   | Port | Status      |
| -------------- | ---------------------------- | ---- | ----------- |
| Frontend       | React 19 + TypeScript + Vite | 5173 | ✅          |
| Backend API    | Rust / Axum 0.7              | 3000 | ✅          |
| Runner Service | Rust / Axum + Docker         | 3001 | 🚧 Skeleton |
| Database       | PostgreSQL 16                | 5432 | ✅          |
| Cache          | Redis 7                      | 6379 | ✅          |
| Object Storage | MinIO (S3-compatible)        | 9000 | ✅          |

## What's Implemented

### ✅ Phase 0: Monorepo Bootstrap

- Vite + React 19 + TypeScript frontend with Tailwind CSS, shadcn/ui, Zustand, TanStack Query, React Router
- Rust/Axum backend with SeaORM, JWT auth, Argon2 password hashing
- Runner microservice skeleton with Docker executor (placeholder)
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
- Frontend: Login & Register pages with Zod validation, react-hook-form
- Frontend: Zustand auth store with localStorage persistence
- Frontend: API service with auto-refresh on 401
- Frontend: RequireAuth wrapper, Navbar with auth state (avatar, XP, level badge)
- Full marketing landing page (13 sections, Framer Motion animations, responsive)

### 🚧 Phase 2: Curriculum (In Progress)

- Database entities & migrations for quests, levels, tasks, progress tables
- Curriculum seeder binary with manifest.json (8 quests, 65 levels)
- API routes for browsing quests/levels/tasks — _next up_

### 📋 Planned

- Phase 3: Code Execution Engine (Docker sandbox, cargo test runner, syntest AST validation)
- Phase 4: Gamification (XP, levels, streaks, achievements, leaderboard)
- Phase 5: Community (forum, articles, reviews)
- Phase 6: Admin Panel (content management, user management)
- Phase 7: CLI Tool, Polish & Deployment

## Project Structure

```
rustgym/
├── frontend/          # React SPA (Vite + TypeScript + Tailwind)
│   ├── src/
│   │   ├── pages/         # Route-level components (Landing, Login, Register)
│   │   ├── components/    # Reusable UI (layout, editor, quest, gamification)
│   │   ├── stores/        # Zustand state (auth, editor, ui)
│   │   ├── services/      # API client with interceptors
│   │   ├── hooks/         # Custom React hooks
│   │   ├── types/         # Shared TypeScript interfaces
│   │   └── lib/           # Utilities (cn, constants)
│   └── tailwind.config.js # Design tokens
├── backend/           # Rust API server (Axum + SeaORM)
│   ├── src/
│   │   ├── routes/        # HTTP handlers (auth, users)
│   │   ├── services/      # Business logic (auth, email)
│   │   ├── middleware/    # Auth, rate limiting, CORS
│   │   ├── dto/           # Request/response types
│   │   ├── config.rs      # Environment config
│   │   └── error.rs       # AppError with IntoResponse
│   ├── entity/            # SeaORM entities (users, quests, levels, tasks, progress)
│   └── migration/         # Database migrations
├── runner/            # Code execution microservice
│   ├── src/main.rs        # Axum server with /run/test and /run/playground
│   └── Dockerfile         # Multi-stage build with Docker CLI
├── challenges/        # Curriculum content
│   └── manifest.json      # 8 quests, 65 levels definition
├── .github/workflows/ # CI/CD pipelines
├── docker-compose.yml # Local dev infrastructure
└── .env.example       # Environment variable template
```

## Prerequisites

- [Node.js](https://nodejs.org/) 20+ and [pnpm](https://pnpm.io/) 9+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- Git

## Getting Started

### 1. Clone and configure

```bash
git clone <repository-url>
cd rustgym
cp .env.example .env
```

### 2. Start infrastructure

```bash
docker compose up -d postgres redis minio
```

### 3. Run the backend

```bash
cargo run --manifest-path backend/Cargo.toml
```

The backend auto-runs migrations on startup and serves at `http://localhost:3000`.

### 4. Seed curriculum data

```bash
cargo run --manifest-path backend/Cargo.toml --bin seed
```

### 5. Start the frontend

```bash
cd frontend
pnpm install
pnpm dev
```

Open `http://localhost:5173` — you'll see the landing page.

## API Endpoints (Implemented)

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

| Method | Path       | Description            |
| ------ | ---------- | ---------------------- |
| GET    | /me        | Get own profile (auth) |
| PUT    | /me        | Update profile (auth)  |
| GET    | /:username | Get public profile     |

### Health

| Method | Path           | Description    |
| ------ | -------------- | -------------- |
| GET    | /api/v1/health | Service status |

## Development

```bash
# Frontend lint + build
cd frontend && pnpm lint && pnpm build

# Backend check + test
cd backend && cargo clippy -- -D warnings && cargo test

# Runner check
cd runner && cargo clippy -- -D warnings
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
| Database | PostgreSQL 16, Redis 7, MinIO                                                                                                 |
| DevOps   | Docker Compose, GitHub Actions                                                                                                |

## License

All rights reserved.
