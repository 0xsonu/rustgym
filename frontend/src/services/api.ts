import type {
  AuthResponse,
  Quest,
  QuestDetail,
  LevelDetail,
  Task,
  RunResponse,
  SubmitResponse,
  Achievement,
  LeaderboardResponse,
  DashboardData,
} from '@/types';

const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api/v1';

let accessToken: string | null = null;
let refreshToken: string | null = null;
let isRefreshing = false;
let refreshPromise: Promise<boolean> | null = null;

export function setTokens(access: string | null, refresh: string | null) {
  accessToken = access;
  refreshToken = refresh;
}

export function getAccessToken() {
  return accessToken;
}

async function request<T>(path: string, options: RequestInit = {}, retry = true): Promise<T> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string>),
  };

  if (accessToken) {
    headers['Authorization'] = `Bearer ${accessToken}`;
  }

  const response = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers,
  });

  if (response.status === 401 && retry && refreshToken) {
    const refreshed = await attemptRefresh();
    if (refreshed) {
      return request<T>(path, options, false);
    }
  }

  if (!response.ok) {
    const body = await response.json().catch(() => ({ message: 'Request failed' }));
    throw { message: body.message || 'Request failed', status: response.status };
  }

  return response.json() as Promise<T>;
}

async function attemptRefresh(): Promise<boolean> {
  if (isRefreshing && refreshPromise) {
    return refreshPromise;
  }

  isRefreshing = true;
  refreshPromise = (async () => {
    try {
      const response = await fetch(`${BASE_URL}/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refresh_token: refreshToken }),
      });

      if (!response.ok) {
        setTokens(null, null);
        return false;
      }

      const data = (await response.json()) as AuthResponse;
      setTokens(data.access_token, data.refresh_token);
      return true;
    } catch {
      setTokens(null, null);
      return false;
    } finally {
      isRefreshing = false;
      refreshPromise = null;
    }
  })();

  return refreshPromise;
}

export const authApi = {
  register(username: string, email: string, password: string) {
    return request<AuthResponse>('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ username, email, password }),
    });
  },

  login(email: string, password: string) {
    return request<AuthResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  },

  refresh() {
    return request<AuthResponse>('/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token: refreshToken }),
    });
  },

  logout() {
    return request<{ message: string }>('/auth/logout', {
      method: 'POST',
      body: JSON.stringify({ refresh_token: refreshToken }),
    });
  },
};

// Quest API
export const questApi = {
  list() {
    return request<Quest[]>('/quests');
  },

  getBySlug(slug: string) {
    return request<QuestDetail>(`/quests/${slug}`);
  },
};

// Level API
export const levelApi = {
  getBySlug(questSlug: string, levelSlug: string) {
    return request<LevelDetail>(`/quests/${questSlug}/levels/${levelSlug}`);
  },
};

// Task API
export const taskApi = {
  getBySlug(slug: string) {
    return request<Task>(`/tasks/${slug}`);
  },

  run(slug: string, code: string) {
    return request<RunResponse>(`/tasks/${slug}/run`, {
      method: 'POST',
      body: JSON.stringify({ code }),
    });
  },

  submit(slug: string, code: string) {
    return request<SubmitResponse>(`/tasks/${slug}/submit`, {
      method: 'POST',
      body: JSON.stringify({ code }),
    });
  },
};

// Achievements API
export const achievementApi = {
  list() {
    return request<Achievement[]>('/achievements');
  },

  getUserAchievements() {
    return request<Achievement[]>('/users/me/achievements');
  },
};

// Leaderboard API
export const leaderboardApi = {
  get(period: 'alltime' | 'weekly' | 'monthly' = 'alltime') {
    return request<LeaderboardResponse>(`/users/leaderboard?period=${period}`);
  },
};

// Dashboard API
export const dashboardApi = {
  get() {
    return request<DashboardData>('/users/me/dashboard');
  },
};
