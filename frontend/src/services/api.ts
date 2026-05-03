import type {
  AuthResponse,
  Quest,
  QuestDetail,
  Level,
  LevelDetail,
  Task,
  RunResponse,
  SubmitResponse,
  Achievement,
  LeaderboardResponse,
  DashboardData,
  ForumPostListResponse,
  ForumPostDetail,
  ForumPostSummary,
  ForumReply,
  ArticleListResponse,
  ArticleDetail,
  ReviewListResponse,
  Review,
  UserProfile,
  PublicProfile,
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
  return accessToken || localStorage.getItem('rustgym_access_token');
}

function getEffectiveTokens() {
  const at = accessToken || localStorage.getItem('rustgym_access_token');
  const rt = refreshToken || localStorage.getItem('rustgym_refresh_token');
  return { accessToken: at, refreshToken: rt };
}

async function request<T>(path: string, options: RequestInit = {}, retry = true): Promise<T> {
  const tokens = getEffectiveTokens();

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string>),
  };

  if (tokens.accessToken) {
    headers['Authorization'] = `Bearer ${tokens.accessToken}`;
  }

  const response = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers,
  });

  if (response.status === 401 && retry && tokens.refreshToken) {
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
      const tokens = getEffectiveTokens();
      if (!tokens.refreshToken) {
        return false;
      }

      const response = await fetch(`${BASE_URL}/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refresh_token: tokens.refreshToken }),
      });

      if (!response.ok) {
        setTokens(null, null);
        localStorage.removeItem('rustgym_access_token');
        localStorage.removeItem('rustgym_refresh_token');
        return false;
      }

      const data = (await response.json()) as AuthResponse;
      setTokens(data.access_token, data.refresh_token);
      // Persist to localStorage so page refreshes work
      localStorage.setItem('rustgym_access_token', data.access_token);
      localStorage.setItem('rustgym_refresh_token', data.refresh_token);
      return true;
    } catch {
      setTokens(null, null);
      localStorage.removeItem('rustgym_access_token');
      localStorage.removeItem('rustgym_refresh_token');
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
  async list() {
    const resp = await request<{ quests: Quest[] }>('/quests');
    return resp.quests;
  },

  async getBySlug(slug: string) {
    const resp = await request<{ quest: Quest; levels: Level[] }>(`/quests/${slug}`);
    return { ...resp.quest, levels: resp.levels } as QuestDetail;
  },
};

// Level API
export const levelApi = {
  async getBySlug(questSlug: string, levelSlug: string) {
    const resp = await request<{ level: Level; tasks: Task[] }>(
      `/quests/${questSlug}/levels/${levelSlug}`,
    );
    return {
      ...resp.level,
      tasks: resp.tasks,
      quest_slug: questSlug,
      quest_title: '',
    } as LevelDetail;
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

  async getUserAchievements() {
    const resp = await request<{
      achievements: Achievement[];
      total_earned: number;
      total_available: number;
    }>('/users/me/achievements');
    return resp.achievements;
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

// Forum API
export const forumApi = {
  listPosts(params?: { page?: number; per_page?: number; task_id?: string; category?: string }) {
    const searchParams = new URLSearchParams();
    if (params?.page) searchParams.set('page', String(params.page));
    if (params?.per_page) searchParams.set('per_page', String(params.per_page));
    if (params?.task_id) searchParams.set('task_id', params.task_id);
    if (params?.category) searchParams.set('category', params.category);
    const qs = searchParams.toString();
    return request<ForumPostListResponse>(`/forum/posts${qs ? `?${qs}` : ''}`);
  },

  getPost(id: string) {
    return request<ForumPostDetail>(`/forum/posts/${id}`);
  },

  createPost(data: { title: string; body_md: string; task_id?: string; category?: string }) {
    return request<ForumPostSummary>('/forum/posts', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  createReply(postId: string, body_md: string) {
    return request<ForumReply>(`/forum/posts/${postId}/reply`, {
      method: 'POST',
      body: JSON.stringify({ body_md }),
    });
  },

  votePost(postId: string) {
    return request<{ votes: number }>(`/forum/posts/${postId}/vote`, {
      method: 'PUT',
    });
  },

  acceptReply(replyId: string) {
    return request<{ is_accepted: boolean }>(`/forum/replies/${replyId}/accept`, {
      method: 'PUT',
    });
  },
};

// Articles API
export const articlesApi = {
  list(params?: { page?: number; per_page?: number }) {
    const searchParams = new URLSearchParams();
    if (params?.page) searchParams.set('page', String(params.page));
    if (params?.per_page) searchParams.set('per_page', String(params.per_page));
    const qs = searchParams.toString();
    return request<ArticleListResponse>(`/articles${qs ? `?${qs}` : ''}`);
  },

  get(id: string) {
    return request<ArticleDetail>(`/articles/${id}`);
  },

  create(data: {
    title: string;
    body_md: string;
    cover_image_url?: string;
    tags?: string[];
    is_published?: boolean;
  }) {
    return request<ArticleDetail>('/articles', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  update(
    id: string,
    data: {
      title?: string;
      body_md?: string;
      cover_image_url?: string;
      tags?: string[];
      is_published?: boolean;
    },
  ) {
    return request<ArticleDetail>(`/articles/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
};

// Reviews API
export const reviewsApi = {
  list() {
    return request<ReviewListResponse>('/reviews');
  },

  create(data: { rating: number; body: string }) {
    return request<Review>('/reviews', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
};

// Profile API
export const profileApi = {
  get(username: string) {
    return request<PublicProfile>(`/users/${username}`);
  },

  updateMe(data: { username?: string; bio?: string }) {
    return request<UserProfile>('/users/me', {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
};
