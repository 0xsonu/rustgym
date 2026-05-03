import type {
  AdminStats,
  AdminQuest,
  AdminLevel,
  AdminTask,
  AdminAchievement,
  AdminUserListResponse,
  AdminUser,
  AdminSubmissionListResponse,
} from '@/types/admin';

import type { AuthResponse } from '@/types';

const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api/v1';

let isRefreshing = false;
let refreshPromise: Promise<boolean> | null = null;

function getToken(): string | null {
  return localStorage.getItem('rustgym_access_token');
}

function getRefreshToken(): string | null {
  return localStorage.getItem('rustgym_refresh_token');
}

async function attemptAdminRefresh(): Promise<boolean> {
  if (isRefreshing && refreshPromise) {
    return refreshPromise;
  }

  isRefreshing = true;
  refreshPromise = (async () => {
    try {
      const rt = getRefreshToken();
      if (!rt) {
        return false;
      }

      const response = await fetch(`${BASE_URL}/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refresh_token: rt }),
      });

      if (!response.ok) {
        localStorage.removeItem('rustgym_access_token');
        localStorage.removeItem('rustgym_refresh_token');
        return false;
      }

      const data = (await response.json()) as AuthResponse;
      localStorage.setItem('rustgym_access_token', data.access_token);
      localStorage.setItem('rustgym_refresh_token', data.refresh_token);
      return true;
    } catch {
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

async function adminRequest<T>(path: string, options: RequestInit = {}, retry = true): Promise<T> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string>),
  };

  const token = getToken();
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const response = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers,
  });

  if (response.status === 401 && retry && getRefreshToken()) {
    const refreshed = await attemptAdminRefresh();
    if (refreshed) {
      return adminRequest<T>(path, options, false);
    }
  }

  if (!response.ok) {
    const body = await response.json().catch(() => ({ message: 'Request failed' }));
    throw { message: body.error?.message || 'Request failed', status: response.status };
  }

  if (response.status === 204) {
    return undefined as unknown as T;
  }

  return response.json() as Promise<T>;
}

export const adminApi = {
  // Stats
  getStats() {
    return adminRequest<AdminStats>('/admin/stats');
  },

  // Quests
  listQuests() {
    return adminRequest<AdminQuest[]>('/admin/quests');
  },
  createQuest(data: Partial<AdminQuest>) {
    return adminRequest<AdminQuest>('/admin/quests', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  updateQuest(id: string, data: Partial<AdminQuest>) {
    return adminRequest<AdminQuest>(`/admin/quests/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
  deleteQuest(id: string) {
    return adminRequest<void>(`/admin/quests/${id}`, { method: 'DELETE' });
  },

  // Levels
  listLevels() {
    return adminRequest<AdminLevel[]>('/admin/levels');
  },
  createLevel(data: Partial<AdminLevel>) {
    return adminRequest<AdminLevel>('/admin/levels', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  updateLevel(id: string, data: Partial<AdminLevel>) {
    return adminRequest<AdminLevel>(`/admin/levels/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
  deleteLevel(id: string) {
    return adminRequest<void>(`/admin/levels/${id}`, { method: 'DELETE' });
  },

  // Tasks
  listTasks() {
    return adminRequest<AdminTask[]>('/admin/tasks');
  },
  createTask(data: Partial<AdminTask>) {
    return adminRequest<AdminTask>('/admin/tasks', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  updateTask(id: string, data: Partial<AdminTask>) {
    return adminRequest<AdminTask>(`/admin/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
  deleteTask(id: string) {
    return adminRequest<void>(`/admin/tasks/${id}`, { method: 'DELETE' });
  },

  // Achievements
  listAchievements() {
    return adminRequest<AdminAchievement[]>('/admin/achievements');
  },
  createAchievement(data: Partial<AdminAchievement>) {
    return adminRequest<AdminAchievement>('/admin/achievements', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
  updateAchievement(id: string, data: Partial<AdminAchievement>) {
    return adminRequest<AdminAchievement>(`/admin/achievements/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },

  // Users
  listUsers(params?: { page?: number; per_page?: number; search?: string }) {
    const searchParams = new URLSearchParams();
    if (params?.page) searchParams.set('page', String(params.page));
    if (params?.per_page) searchParams.set('per_page', String(params.per_page));
    if (params?.search) searchParams.set('search', params.search);
    const qs = searchParams.toString();
    return adminRequest<AdminUserListResponse>(`/admin/users${qs ? `?${qs}` : ''}`);
  },
  updateUserRole(id: string, role: string) {
    return adminRequest<AdminUser>(`/admin/users/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ role }),
    });
  },
  banUser(id: string) {
    return adminRequest<void>(`/admin/users/${id}`, { method: 'DELETE' });
  },

  // Submissions
  listSubmissions(params?: { page?: number; per_page?: number }) {
    const searchParams = new URLSearchParams();
    if (params?.page) searchParams.set('page', String(params.page));
    if (params?.per_page) searchParams.set('per_page', String(params.per_page));
    const qs = searchParams.toString();
    return adminRequest<AdminSubmissionListResponse>(`/admin/submissions${qs ? `?${qs}` : ''}`);
  },
};
