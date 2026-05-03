#!/usr/bin/env bash
# =============================================================================
# RustGym — Local Development Startup Script
# =============================================================================
#
# Starts all services needed for local development:
#   1. Docker infrastructure (PostgreSQL, Redis, MinIO)
#   2. Backend API (Rust/Axum on port 3000)
#   3. Runner service (code execution sandbox on port 3001)
#   4. Frontend dev server (Vite on port 5173)
#
# Usage:
#   bash dev.sh          # Start everything
#   bash dev.sh stop     # Stop all services
#   bash dev.sh seed     # Seed curriculum data
#   bash dev.sh logs     # Tail backend logs
#
# Prerequisites:
#   - Docker and Docker Compose
#   - Rust toolchain (cargo)
#   - Node.js 20+ and pnpm 9+
#   - .env file (copy from .env.example if missing)
#
# =============================================================================

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

log() { echo -e "${CYAN}[dev]${NC} $1"; }
success() { echo -e "${GREEN}[dev]${NC} $1"; }
warn() { echo -e "${YELLOW}[dev]${NC} $1"; }
error() { echo -e "${RED}[dev]${NC} $1"; }

# =============================================================================
# Commands
# =============================================================================

cmd_stop() {
    log "Stopping all services..."
    docker compose down 2>/dev/null || true

    # Kill background processes
    if [ -f .dev-pids ]; then
        while read -r pid; do
            kill "$pid" 2>/dev/null || true
        done < .dev-pids
        rm -f .dev-pids
    fi

    success "All services stopped."
}

cmd_seed() {
    log "Seeding curriculum data..."
    cargo run --manifest-path backend/Cargo.toml --bin seed
    success "Seed complete!"
}

cmd_logs() {
    if [ -f .dev-backend.log ]; then
        tail -f .dev-backend.log
    else
        error "No backend log file found. Is the backend running?"
        exit 1
    fi
}

cmd_start() {
    # ─── Preflight checks ────────────────────────────────────────────────────
    log "Running preflight checks..."

    if ! command -v docker &> /dev/null; then
        error "Docker is not installed. Please install Docker first."
        exit 1
    fi

    if ! command -v cargo &> /dev/null; then
        error "Rust toolchain not found. Install from https://rustup.rs"
        exit 1
    fi

    if ! command -v pnpm &> /dev/null; then
        error "pnpm not found. Install with: npm install -g pnpm"
        exit 1
    fi

    # ─── Environment ─────────────────────────────────────────────────────────
    if [ ! -f .env ]; then
        if [ -f .env.example ]; then
            cp .env.example .env
            warn "Created .env from .env.example"
        else
            error "No .env file found. Create one from .env.example"
            exit 1
        fi
    fi

    # ─── Stop any existing services ──────────────────────────────────────────
    cmd_stop 2>/dev/null || true
    rm -f .dev-pids .dev-backend.log .dev-frontend.log .dev-runner.log

    # ─── Start Docker infrastructure ─────────────────────────────────────────
    log "Starting Docker infrastructure (PostgreSQL, Redis, MinIO)..."
    docker compose up -d postgres redis minio

    # Wait for Postgres to be ready
    log "Waiting for PostgreSQL to be ready..."
    for i in $(seq 1 30); do
        if docker compose exec -T postgres pg_isready -U rustgym > /dev/null 2>&1; then
            break
        fi
        if [ "$i" -eq 30 ]; then
            error "PostgreSQL failed to start within 30 seconds"
            exit 1
        fi
        sleep 1
    done
    success "PostgreSQL is ready"

    # Wait for Redis
    log "Waiting for Redis to be ready..."
    for i in $(seq 1 15); do
        if docker compose exec -T redis redis-cli ping > /dev/null 2>&1; then
            break
        fi
        if [ "$i" -eq 15 ]; then
            warn "Redis not ready (rate limiting will be disabled)"
        fi
        sleep 1
    done
    success "Redis is ready"

    # ─── Install frontend dependencies ───────────────────────────────────────
    if [ ! -d frontend/node_modules ]; then
        log "Installing frontend dependencies..."
        (cd frontend && pnpm install)
    fi

    # ─── Start Backend ───────────────────────────────────────────────────────
    log "Starting backend API (port 3000)..."
    cargo run --manifest-path backend/Cargo.toml --bin rustgym-backend > .dev-backend.log 2>&1 &
    BACKEND_PID=$!
    echo "$BACKEND_PID" >> .dev-pids

    # Wait for backend to be ready
    for i in $(seq 1 30); do
        if curl -s http://localhost:3000/api/v1/health > /dev/null 2>&1; then
            break
        fi
        # Check if process died
        if ! kill -0 "$BACKEND_PID" 2>/dev/null; then
            error "Backend failed to start. Check .dev-backend.log for details:"
            tail -20 .dev-backend.log
            exit 1
        fi
        if [ "$i" -eq 30 ]; then
            error "Backend failed to respond within 30 seconds"
            tail -10 .dev-backend.log
            exit 1
        fi
        sleep 1
    done
    success "Backend API running at http://localhost:3000"

    # ─── Start Runner Service ────────────────────────────────────────────────
    log "Starting runner service (port 3001)..."
    cargo run --manifest-path runner/Cargo.toml > .dev-runner.log 2>&1 &
    RUNNER_PID=$!
    echo "$RUNNER_PID" >> .dev-pids

    # Wait for runner to be ready
    for i in $(seq 1 30); do
        if curl -s http://localhost:3001/health > /dev/null 2>&1; then
            break
        fi
        # Check if process died
        if ! kill -0 "$RUNNER_PID" 2>/dev/null; then
            error "Runner failed to start. Check .dev-runner.log for details:"
            tail -20 .dev-runner.log
            exit 1
        fi
        if [ "$i" -eq 30 ]; then
            error "Runner failed to respond within 30 seconds"
            tail -10 .dev-runner.log
            exit 1
        fi
        sleep 1
    done
    success "Runner service running at http://localhost:3001"

    # ─── Start Frontend ──────────────────────────────────────────────────────
    log "Starting frontend dev server (port 5173)..."
    (cd frontend && pnpm dev) > .dev-frontend.log 2>&1 &
    FRONTEND_PID=$!
    echo "$FRONTEND_PID" >> .dev-pids

    # Wait for frontend
    for i in $(seq 1 15); do
        if curl -s http://localhost:5173 > /dev/null 2>&1; then
            break
        fi
        sleep 1
    done
    success "Frontend running at http://localhost:5173"

    # ─── Summary ─────────────────────────────────────────────────────────────
    echo ""
    echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}  🦀 RustGym Development Environment${NC}"
    echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "  ${GREEN}Frontend${NC}:    http://localhost:5173"
    echo -e "  ${GREEN}Backend API${NC}: http://localhost:3000"
    echo -e "  ${GREEN}Runner${NC}:      http://localhost:3001"
    echo -e "  ${GREEN}API Docs${NC}:    http://localhost:3000/api/docs"
    echo -e "  ${GREEN}Health${NC}:      http://localhost:3000/api/v1/health"
    echo ""
    echo -e "  ${CYAN}PostgreSQL${NC}:  localhost:5432 (rustgym/rustgym_dev)"
    echo -e "  ${CYAN}Redis${NC}:       localhost:6379"
    echo -e "  ${CYAN}MinIO${NC}:       localhost:9000 (console: 9001)"
    echo ""
    echo -e "  ${YELLOW}Commands${NC}:"
    echo -e "    bash dev.sh stop    — Stop all services"
    echo -e "    bash dev.sh seed    — Seed curriculum data"
    echo -e "    bash dev.sh logs    — Tail backend logs"
    echo ""
    echo -e "  ${YELLOW}Logs${NC}:"
    echo -e "    Backend:  tail -f .dev-backend.log"
    echo -e "    Runner:   tail -f .dev-runner.log"
    echo -e "    Frontend: tail -f .dev-frontend.log"
    echo ""
    echo -e "  Press ${BOLD}Ctrl+C${NC} to stop all services."
    echo ""

    # ─── Wait for Ctrl+C ────────────────────────────────────────────────────
    trap 'echo ""; log "Shutting down..."; cmd_stop; exit 0' INT TERM

    # Keep script running and forward signals
    wait
}

# =============================================================================
# Entry point
# =============================================================================

case "${1:-start}" in
    start)  cmd_start ;;
    stop)   cmd_stop ;;
    seed)   cmd_seed ;;
    logs)   cmd_logs ;;
    *)
        echo "Usage: bash dev.sh [start|stop|seed|logs]"
        exit 1
        ;;
esac
