// Shared TypeScript types and interfaces

export interface UserProfile {
  id: string;
  username: string;
  email: string;
  avatar_url: string | null;
  bio: string | null;
  role: 'student' | 'mentor' | 'admin';
  xp: number;
  level: number;
  streak_days: number;
  is_verified: boolean;
  created_at: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  user: UserProfile;
}

export interface ApiError {
  message: string;
  status?: number;
}
