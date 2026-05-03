/**
 * Preservation Property Tests - Frontend API Client
 *
 * These tests verify that existing working API client behavior is preserved
 * BEFORE and AFTER the bugfix is applied. They should PASS on both unfixed
 * and fixed code.
 *
 * **Validates: Requirements 3.5, 3.1 (auth flow preservation)**
 *
 * Observation-first methodology:
 * - Observe: main api.ts request function attempts refresh on 401
 * - Observe: authApi methods (login, register, logout, refresh) maintain request shapes
 * - Write tests asserting these behaviors are preserved
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ─── Property: api.ts request function attempts refresh on 401 ───────────────

/**
 * **Validates: Requirements 3.5**
 *
 * Property: main api.ts request function still attempts refresh on 401
 * (existing behavior preserved).
 *
 * Observation: The request() function in api.ts checks for 401 status,
 * and if a refresh token is available, calls attemptRefresh() which
 * POSTs to /auth/refresh with the stored refresh token.
 */
describe('Preservation: api.ts token refresh on 401', () => {
  beforeEach(() => {
    vi.resetModules();
    vi.stubGlobal('localStorage', {
      getItem: vi.fn((key: string) => {
        if (key === 'rustgym_access_token') return 'expired_access_token';
        if (key === 'rustgym_refresh_token') return 'valid_refresh_token';
        return null;
      }),
      setItem: vi.fn(),
      removeItem: vi.fn(),
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it('should attempt token refresh when receiving 401 and refresh token exists', async () => {
    let callCount = 0;
    const fetchCalls: { url: string; method?: string }[] = [];

    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, options?: RequestInit) => {
        callCount++;
        fetchCalls.push({ url, method: options?.method });

        if (callCount === 1) {
          // First call: return 401 to trigger refresh
          return new Response(JSON.stringify({ message: 'Token expired' }), {
            status: 401,
            headers: { 'Content-Type': 'application/json' },
          });
        }

        if (url.includes('/auth/refresh')) {
          // Refresh call: return new tokens
          return new Response(
            JSON.stringify({
              access_token: 'new_access_token',
              refresh_token: 'new_refresh_token',
              user: {
                id: '550e8400-e29b-41d4-a716-446655440000',
                username: 'testuser',
                email: 'test@example.com',
                avatar_url: null,
                bio: null,
                role: 'student',
                xp: 100,
                level: 2,
                streak_days: 0,
                is_verified: true,
                created_at: '2024-01-01T00:00:00Z',
              },
            }),
            { status: 200, headers: { 'Content-Type': 'application/json' } },
          );
        }

        // Retry call: return success
        return new Response(
          JSON.stringify({
            quests: [
              {
                id: '550e8400-e29b-41d4-a716-446655440000',
                slug: 'rust-basics',
                title: 'Rust Basics',
                description: 'Learn Rust',
                icon: '🦀',
                color: '#FF6B35',
                order_index: 1,
                level_count: 3,
                task_count: 15,
                is_published: true,
                prerequisite_quest_id: null,
              },
            ],
          }),
          { status: 200, headers: { 'Content-Type': 'application/json' } },
        );
      }),
    );

    const { questApi } = await import('@/services/api');
    const result = await questApi.list();

    // Verify refresh was attempted
    expect(callCount).toBeGreaterThanOrEqual(3); // original + refresh + retry
    const refreshCall = fetchCalls.find((c) => c.url.includes('/auth/refresh'));
    expect(refreshCall).toBeDefined();
    expect(refreshCall?.method).toBe('POST');

    // Verify the final result is correct
    expect(result).toBeDefined();
    expect(Array.isArray(result)).toBe(true);
  });

  it('should clear tokens when refresh fails', async () => {
    let callCount = 0;

    vi.stubGlobal(
      'fetch',
      vi.fn(async () => {
        callCount++;

        if (callCount === 1) {
          // First call: return 401
          return new Response(JSON.stringify({ message: 'Token expired' }), {
            status: 401,
            headers: { 'Content-Type': 'application/json' },
          });
        }

        // Refresh call: also fails
        return new Response(JSON.stringify({ message: 'Refresh token invalid' }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        });
      }),
    );

    const { questApi } = await import('@/services/api');

    // Should throw because refresh failed
    await expect(questApi.list()).rejects.toMatchObject({
      status: 401,
    });

    // Verify tokens were cleared from localStorage
    expect(localStorage.removeItem).toHaveBeenCalledWith('rustgym_access_token');
    expect(localStorage.removeItem).toHaveBeenCalledWith('rustgym_refresh_token');
  });
});

// ─── Property: authApi methods maintain their request shapes ─────────────────

/**
 * **Validates: Requirements 3.5**
 *
 * Property: authApi methods (login, register, logout, refresh) maintain
 * their request shapes.
 *
 * Observation: authApi methods send specific JSON bodies:
 * - register: { username, email, password }
 * - login: { email, password }
 * - refresh: { refresh_token }
 * - logout: { refresh_token }
 */
describe('Preservation: authApi request shapes', () => {
  beforeEach(() => {
    vi.resetModules();
    vi.stubGlobal('localStorage', {
      getItem: vi.fn(() => null),
      setItem: vi.fn(),
      removeItem: vi.fn(),
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it('register sends { username, email, password } via POST', async () => {
    let capturedBody: string | undefined;
    let capturedMethod: string | undefined;
    let capturedUrl: string | undefined;

    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, options?: RequestInit) => {
        capturedUrl = url;
        capturedMethod = options?.method;
        capturedBody = options?.body as string;
        return new Response(
          JSON.stringify({
            access_token: 'at',
            refresh_token: 'rt',
            user: {
              id: '1',
              username: 'newuser',
              email: 'new@example.com',
              avatar_url: null,
              bio: null,
              role: 'student',
              xp: 0,
              level: 1,
              streak_days: 0,
              is_verified: false,
              created_at: '2024-01-01T00:00:00Z',
            },
          }),
          { status: 200, headers: { 'Content-Type': 'application/json' } },
        );
      }),
    );

    const { authApi } = await import('@/services/api');
    await authApi.register('newuser', 'new@example.com', 'password123');

    expect(capturedUrl).toContain('/auth/register');
    expect(capturedMethod).toBe('POST');
    const body = JSON.parse(capturedBody!);
    expect(body).toEqual({
      username: 'newuser',
      email: 'new@example.com',
      password: 'password123',
    });
  });

  it('login sends { email, password } via POST', async () => {
    let capturedBody: string | undefined;
    let capturedMethod: string | undefined;
    let capturedUrl: string | undefined;

    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, options?: RequestInit) => {
        capturedUrl = url;
        capturedMethod = options?.method;
        capturedBody = options?.body as string;
        return new Response(
          JSON.stringify({
            access_token: 'at',
            refresh_token: 'rt',
            user: {
              id: '1',
              username: 'testuser',
              email: 'test@example.com',
              avatar_url: null,
              bio: null,
              role: 'student',
              xp: 100,
              level: 2,
              streak_days: 5,
              is_verified: true,
              created_at: '2024-01-01T00:00:00Z',
            },
          }),
          { status: 200, headers: { 'Content-Type': 'application/json' } },
        );
      }),
    );

    const { authApi } = await import('@/services/api');
    await authApi.login('test@example.com', 'password123');

    expect(capturedUrl).toContain('/auth/login');
    expect(capturedMethod).toBe('POST');
    const body = JSON.parse(capturedBody!);
    expect(body).toEqual({
      email: 'test@example.com',
      password: 'password123',
    });
  });

  it('logout sends { refresh_token } via POST', async () => {
    let capturedBody: string | undefined;
    let capturedMethod: string | undefined;
    let capturedUrl: string | undefined;

    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, options?: RequestInit) => {
        capturedUrl = url;
        capturedMethod = options?.method;
        capturedBody = options?.body as string;
        return new Response(JSON.stringify({ message: 'Logged out' }), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        });
      }),
    );

    const { authApi, setTokens } = await import('@/services/api');
    setTokens('access_token', 'my_refresh_token');
    await authApi.logout();

    expect(capturedUrl).toContain('/auth/logout');
    expect(capturedMethod).toBe('POST');
    const body = JSON.parse(capturedBody!);
    expect(body).toHaveProperty('refresh_token');
  });

  it('refresh sends { refresh_token } via POST', async () => {
    let capturedBody: string | undefined;
    let capturedMethod: string | undefined;
    let capturedUrl: string | undefined;

    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, options?: RequestInit) => {
        capturedUrl = url;
        capturedMethod = options?.method;
        capturedBody = options?.body as string;
        return new Response(
          JSON.stringify({
            access_token: 'new_at',
            refresh_token: 'new_rt',
            user: {
              id: '1',
              username: 'testuser',
              email: 'test@example.com',
              avatar_url: null,
              bio: null,
              role: 'student',
              xp: 100,
              level: 2,
              streak_days: 5,
              is_verified: true,
              created_at: '2024-01-01T00:00:00Z',
            },
          }),
          { status: 200, headers: { 'Content-Type': 'application/json' } },
        );
      }),
    );

    const { authApi, setTokens } = await import('@/services/api');
    setTokens('old_access', 'old_refresh');
    await authApi.refresh();

    expect(capturedUrl).toContain('/auth/refresh');
    expect(capturedMethod).toBe('POST');
    const body = JSON.parse(capturedBody!);
    expect(body).toHaveProperty('refresh_token');
  });
});
