import { create } from 'zustand';
import type { UserProfile, ApiError } from '@/types';
import { authApi, setTokens } from '@/services/api';

interface AuthState {
  accessToken: string | null;
  refreshToken: string | null;
  user: UserProfile | null;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (username: string, email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  refresh: () => Promise<void>;
  setUser: (user: UserProfile | null) => void;
}

function loadTokens() {
  try {
    const accessToken = localStorage.getItem('rustgym_access_token');
    const refreshToken = localStorage.getItem('rustgym_refresh_token');
    const userJson = localStorage.getItem('rustgym_user');
    const user = userJson ? (JSON.parse(userJson) as UserProfile) : null;

    if (accessToken && refreshToken) {
      setTokens(accessToken, refreshToken);
    }

    return { accessToken, refreshToken, user, isAuthenticated: !!accessToken && !!user };
  } catch {
    return { accessToken: null, refreshToken: null, user: null, isAuthenticated: false };
  }
}

function persistTokens(
  accessToken: string | null,
  refreshToken: string | null,
  user: UserProfile | null,
) {
  if (accessToken && refreshToken) {
    localStorage.setItem('rustgym_access_token', accessToken);
    localStorage.setItem('rustgym_refresh_token', refreshToken);
  } else {
    localStorage.removeItem('rustgym_access_token');
    localStorage.removeItem('rustgym_refresh_token');
  }

  if (user) {
    localStorage.setItem('rustgym_user', JSON.stringify(user));
  } else {
    localStorage.removeItem('rustgym_user');
  }
}

const initialState = loadTokens();

export const useAuthStore = create<AuthState>((set) => ({
  accessToken: initialState.accessToken,
  refreshToken: initialState.refreshToken,
  user: initialState.user,
  isAuthenticated: initialState.isAuthenticated,

  async login(email: string, password: string) {
    const data = await authApi.login(email, password);
    setTokens(data.access_token, data.refresh_token);
    persistTokens(data.access_token, data.refresh_token, data.user);
    set({
      accessToken: data.access_token,
      refreshToken: data.refresh_token,
      user: data.user,
      isAuthenticated: true,
    });
  },

  async register(username: string, email: string, password: string) {
    const data = await authApi.register(username, email, password);
    setTokens(data.access_token, data.refresh_token);
    persistTokens(data.access_token, data.refresh_token, data.user);
    set({
      accessToken: data.access_token,
      refreshToken: data.refresh_token,
      user: data.user,
      isAuthenticated: true,
    });
  },

  async logout() {
    try {
      await authApi.logout();
    } catch {
      // Logout even if API call fails
    }
    setTokens(null, null);
    persistTokens(null, null, null);
    set({
      accessToken: null,
      refreshToken: null,
      user: null,
      isAuthenticated: false,
    });
  },

  async refresh() {
    try {
      const data = await authApi.refresh();
      setTokens(data.access_token, data.refresh_token);
      persistTokens(data.access_token, data.refresh_token, data.user);
      set({
        accessToken: data.access_token,
        refreshToken: data.refresh_token,
        user: data.user,
        isAuthenticated: true,
      });
    } catch (error: unknown) {
      const apiError = error as ApiError;
      setTokens(null, null);
      persistTokens(null, null, null);
      set({
        accessToken: null,
        refreshToken: null,
        user: null,
        isAuthenticated: false,
      });
      throw apiError;
    }
  },

  setUser(user: UserProfile | null) {
    if (user) {
      localStorage.setItem('rustgym_user', JSON.stringify(user));
    } else {
      localStorage.removeItem('rustgym_user');
    }
    set({ user, isAuthenticated: !!user });
  },
}));
