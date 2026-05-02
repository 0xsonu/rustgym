#!/usr/bin/env bash
# =============================================================================
# RustGym Docker Compose Validation Script
# =============================================================================
#
# Validates:
#   1. docker-compose.prod.yml is valid YAML (via docker compose config)
#   2. All required environment variables are documented
#
# Prerequisites:
#   - Docker and docker compose installed
#   - Run from the rustgym/ project root
#
# Usage:
#   bash tests/validate_compose.sh
#
# =============================================================================

set -uo pipefail

PASS=0
FAIL=0
SKIP=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

pass() {
    echo -e "  ${GREEN}✓ PASS${NC}: $1"
    PASS=$((PASS + 1))
}

fail() {
    echo -e "  ${RED}✗ FAIL${NC}: $1"
    [ -n "${2:-}" ] && echo -e "    ${RED}Detail: $2${NC}"
    FAIL=$((FAIL + 1))
}

skip() {
    echo -e "  ${YELLOW}⊘ SKIP${NC}: $1"
    SKIP=$((SKIP + 1))
}

echo "=== RustGym Docker Compose Validation ==="
echo ""

# Determine project root (script is in tests/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

COMPOSE_FILE="$PROJECT_ROOT/docker-compose.prod.yml"
ENV_EXAMPLE="$PROJECT_ROOT/.env.example"

# =============================================================================
# Check 1: docker-compose.prod.yml exists
# =============================================================================
echo "1. Check docker-compose.prod.yml exists"
if [ -f "$COMPOSE_FILE" ]; then
    pass "docker-compose.prod.yml found"
else
    fail "docker-compose.prod.yml not found at $COMPOSE_FILE"
    echo "Cannot continue without compose file."
    exit 1
fi
echo ""

# =============================================================================
# Check 2: Validate YAML with docker compose config
# =============================================================================
echo "2. Validate YAML syntax"
if command -v docker &> /dev/null; then
    # Set dummy env vars so compose config doesn't fail on missing vars
    export POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-dummy_password}"
    export JWT_SECRET="${JWT_SECRET:-dummy_jwt_secret}"
    export FRONTEND_URL="${FRONTEND_URL:-https://rustgym.dev}"
    export S3_BUCKET="${S3_BUCKET:-rustgym}"
    export S3_REGION="${S3_REGION:-us-east-1}"
    export S3_ENDPOINT="${S3_ENDPOINT:-http://minio:9000}"

    VALIDATE_OUTPUT=$(docker compose -f "$COMPOSE_FILE" config 2>&1)
    VALIDATE_EXIT=$?

    if [ $VALIDATE_EXIT -eq 0 ]; then
        pass "docker-compose.prod.yml is valid YAML"
    else
        fail "docker-compose.prod.yml validation failed" "$VALIDATE_OUTPUT"
    fi
else
    skip "Docker not installed — cannot validate compose file"
fi
echo ""

# =============================================================================
# Check 3: All services defined
# =============================================================================
echo "3. Check required services are defined"
REQUIRED_SERVICES=("backend" "frontend" "runner" "postgres" "redis" "nginx")

if command -v docker &> /dev/null; then
    SERVICES=$(docker compose -f "$COMPOSE_FILE" config --services 2>/dev/null)

    for svc in "${REQUIRED_SERVICES[@]}"; do
        if echo "$SERVICES" | grep -q "^${svc}$"; then
            pass "Service '$svc' is defined"
        else
            # sandbox-builder is a build-only service, check separately
            if [ "$svc" = "sandbox-builder" ]; then
                skip "Service '$svc' (build-only, may not appear in services list)"
            else
                fail "Service '$svc' is NOT defined in compose file"
            fi
        fi
    done
else
    skip "Docker not installed — cannot check services"
fi
echo ""

# =============================================================================
# Check 4: Required environment variables documented in .env.example
# =============================================================================
echo "4. Check environment variables are documented"

# These are the variables referenced in docker-compose.prod.yml
# POSTGRES_PASSWORD is production-only (dev uses hardcoded value in docker-compose.yml)
REQUIRED_VARS=("DATABASE_URL" "JWT_SECRET" "FRONTEND_URL" "S3_BUCKET" "S3_REGION" "S3_ENDPOINT" "RUNNER_URL")

if [ -f "$ENV_EXAMPLE" ]; then
    for var in "${REQUIRED_VARS[@]}"; do
        if grep -q "^${var}=" "$ENV_EXAMPLE" || grep -q "^# *${var}" "$ENV_EXAMPLE" || grep -q "${var}" "$ENV_EXAMPLE"; then
            pass "Variable '$var' documented in .env.example"
        else
            fail "Variable '$var' NOT documented in .env.example"
        fi
    done
else
    skip "No .env.example found — cannot verify variable documentation"
fi
echo ""

# =============================================================================
# Check 5: Health checks defined for critical services
# =============================================================================
echo "5. Check health checks are defined"
HEALTHCHECK_SERVICES=("backend" "postgres" "redis")

for svc in "${HEALTHCHECK_SERVICES[@]}"; do
    # Extract the service block using awk (portable across macOS/Linux)
    SERVICE_BLOCK=$(awk "/^  ${svc}:/{found=1; next} found && /^  [a-z]/{exit} found{print}" "$COMPOSE_FILE")
    if echo "$SERVICE_BLOCK" | grep -q "healthcheck:"; then
        pass "Service '$svc' has a healthcheck defined"
    else
        fail "Service '$svc' is missing a healthcheck"
    fi
done
echo ""

# =============================================================================
# Check 6: Resource limits defined
# =============================================================================
echo "6. Check resource limits are defined"
RESOURCE_SERVICES=("backend" "runner" "postgres" "redis")

for svc in "${RESOURCE_SERVICES[@]}"; do
    SERVICE_BLOCK=$(awk "/^  ${svc}:/{found=1; next} found && /^  [a-z]/{exit} found{print}" "$COMPOSE_FILE")
    if echo "$SERVICE_BLOCK" | grep -q "limits:"; then
        pass "Service '$svc' has resource limits"
    else
        fail "Service '$svc' is missing resource limits"
    fi
done
echo ""

# =============================================================================
# Summary
# =============================================================================
echo "=== Validation Results ==="
echo -e "  ${GREEN}Passed${NC}: $PASS"
echo -e "  ${RED}Failed${NC}: $FAIL"
echo -e "  ${YELLOW}Skipped${NC}: $SKIP"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo -e "${RED}VALIDATION FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}VALIDATION PASSED${NC}"
    exit 0
fi
