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

// Progress tracking
export interface UserProgress {
  tasks_completed: number;
  tasks_total: number;
  is_completed: boolean;
}

// Quest types
export interface Quest {
  id: string;
  slug: string;
  title: string;
  description: string;
  icon: string;
  color: string;
  order_index: number;
  level_count: number;
  task_count: number;
  is_published: boolean;
  prerequisite_quest_id: string | null;
  user_progress?: UserProgress;
}

// Level types
export interface Level {
  id: string;
  slug: string;
  title: string;
  description: string;
  quest_id: string;
  order_index: number;
  task_count: number;
  user_progress?: UserProgress;
}

export interface QuestDetail extends Quest {
  levels: Level[];
}

// Task types
export type Difficulty = 'beginner' | 'easy' | 'medium' | 'hard' | 'advanced';

export interface Task {
  id: string;
  slug: string;
  title: string;
  description_md: string;
  starter_code: string;
  difficulty: Difficulty;
  xp_reward: number;
  tags: string[];
  hint_md: string | null;
  order_index: number;
  is_completed?: boolean;
}

export interface LevelDetail extends Level {
  tasks: Task[];
  quest_slug: string;
  quest_title: string;
}

export interface TestResult {
  name: string;
  passed: boolean;
  message: string | null;
}

export interface Submission {
  id: string;
  task_id: string;
  code: string;
  status: 'pending' | 'running' | 'passed' | 'failed' | 'error' | 'timeout';
  test_results: TestResult[];
  stdout: string;
  stderr: string;
  duration_ms: number;
  memory_kb: number;
  xp_awarded: number;
  attempt_number: number;
  created_at: string;
}
