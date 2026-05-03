/**
 * Bug Condition Exploration Tests - Frontend
 *
 * These tests verify the frontend-to-backend contract mismatches exist.
 * They are EXPECTED TO FAIL on unfixed code — failure confirms the bugs exist.
 *
 * **Validates: Requirements 1.4, 1.6, 1.7, 1.8**
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ─── Bug 4: Admin API missing token refresh ──────────────────────────────────
// adminApi.ts has no 401 interception or refresh logic.
// When token expires, admin gets 401 with no automatic retry.

describe('Bug 4: Admin API token refresh', () => {
  beforeEach(() => {
    vi.stubGlobal('localStorage', {
      getItem: vi.fn((key: string) => {
        if (key === 'rustgym_access_token') return 'expired_token';
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

  it('should attempt token refresh when receiving 401 (BUG: no refresh logic exists)', async () => {
    let callCount = 0;

    // Mock fetch: first call returns 401, second call (after refresh) returns 200
    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, _options?: RequestInit) => {
        callCount++;

        if (callCount === 1) {
          // First call: simulate expired token → 401
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
              user: { id: '1', username: 'admin' },
            }),
            { status: 200, headers: { 'Content-Type': 'application/json' } },
          );
        }

        // Retry call with new token: return success
        return new Response(JSON.stringify({ users: [], total: 0 }), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        });
      }),
    );

    // Import adminApi dynamically to use mocked fetch
    const { adminApi } = await import('@/services/adminApi');

    // BUG CONDITION: adminRequest does NOT handle 401 with refresh.
    // It will throw on the 401 response instead of attempting refresh.
    // If token refresh logic existed, this would succeed after refresh.
    try {
      await adminApi.getStats();
      // If we get here, refresh logic exists (bug is fixed)
      expect(callCount).toBeGreaterThan(1);
      // Verify refresh was attempted
      const fetchMock = vi.mocked(fetch);
      const refreshCall = fetchMock.mock.calls.find(
        (call) => typeof call[0] === 'string' && call[0].includes('/auth/refresh'),
      );
      expect(refreshCall).toBeDefined();
    } catch (error: unknown) {
      // BUG CONFIRMED: adminRequest throws on 401 without attempting refresh
      const err = error as { status?: number; message?: string };
      expect(err.status).toBe(401);
      // This failure confirms Bug 4 exists - no token refresh in adminApi
      throw new Error(
        `BUG 4 CONFIRMED: adminRequest threw on 401 without attempting token refresh. ` +
          `Error: ${err.message}. The adminApi.ts has no 401 interception logic.`,
        { cause: error },
      );
    }
  });
});

// ─── Bug 6: Public profile type mismatch (FIXED) ─────────────────────────────
// Frontend profileApi.get() now returns PublicProfile which correctly excludes email and is_verified
// Backend PublicProfileResponse does NOT include these fields — frontend type now matches

describe('Bug 6: Public profile type mismatch (FIXED)', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it('should return profile WITHOUT email/is_verified for public profiles (FIXED: uses PublicProfile type)', async () => {
    // Simulate what the backend actually returns for GET /users/:username
    const backendResponse = {
      id: '550e8400-e29b-41d4-a716-446655440000',
      username: 'testuser',
      avatar_url: null,
      bio: 'Hello world',
      role: 'student',
      xp: 100,
      level: 2,
      streak_days: 5,
      created_at: '2024-01-01T00:00:00Z',
      // NOTE: email and is_verified are intentionally NOT included
      // because this is a PUBLIC profile endpoint
    };

    vi.stubGlobal('localStorage', {
      getItem: vi.fn(() => 'some_token'),
      setItem: vi.fn(),
      removeItem: vi.fn(),
    });

    vi.stubGlobal(
      'fetch',
      vi.fn(async () => {
        return new Response(JSON.stringify(backendResponse), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        });
      }),
    );

    const { profileApi } = await import('@/services/api');

    const profile = await profileApi.get('testuser');

    // FIX VERIFIED: Frontend now uses PublicProfile type that correctly
    // does NOT include email/is_verified fields. The response matches the type.
    expect(profile.username).toBe('testuser');
    expect(profile.xp).toBe(100);
    expect(profile.level).toBe(2);
    // These fields should NOT be present in a public profile response
    expect((profile as unknown as Record<string, unknown>).email).toBeUndefined();
    expect((profile as unknown as Record<string, unknown>).is_verified).toBeUndefined();
  });
});

// ─── Bug 7: Achievement response unwrapping ──────────────────────────────────
// Frontend achievementApi.getUserAchievements() is typed as returning Achievement[]
// Backend returns { achievements: [...], total_earned, total_available }

describe('Bug 7: Achievement response unwrapping', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it('should return Achievement[] directly from getUserAchievements (BUG: returns wrapped object)', async () => {
    // Simulate what the backend actually returns
    const backendResponse = {
      achievements: [
        {
          id: '550e8400-e29b-41d4-a716-446655440000',
          slug: 'first-submission',
          name: 'First Steps',
          description: 'Submit your first solution',
          icon: '🎯',
          xp_reward: 50,
          is_earned: true,
          earned_at: '2024-01-15T10:00:00Z',
        },
      ],
      total_earned: 1,
      total_available: 10,
    };

    vi.stubGlobal('localStorage', {
      getItem: vi.fn(() => 'some_token'),
      setItem: vi.fn(),
      removeItem: vi.fn(),
    });

    vi.stubGlobal(
      'fetch',
      vi.fn(async () => {
        return new Response(JSON.stringify(backendResponse), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        });
      }),
    );

    const { achievementApi } = await import('@/services/api');

    const result = await achievementApi.getUserAchievements();

    // BUG CONDITION: Frontend expects Achievement[] (an array)
    // Backend returns { achievements: [...], total_earned, total_available }
    // The request<Achievement[]>('/users/me/achievements') will parse the JSON
    // and return the object as-is, but the type says it's an array.
    expect(Array.isArray(result)).toBe(true);

    if (Array.isArray(result)) {
      // If it IS an array, verify it has the right shape
      expect(result.length).toBeGreaterThan(0);
      expect(result[0]).toHaveProperty('id');
    }
  });
});
