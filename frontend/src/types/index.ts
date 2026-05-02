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

export interface SubmitResponse {
  id: string;
  status: 'passed' | 'failed' | 'error' | 'timeout';
  test_results: TestResult[];
  stdout: string;
  stderr: string;
  duration_ms: number;
  memory_kb: number;
  xp_awarded: number;
  attempt_number: number;
  leveled_up: boolean;
  new_level: number | null;
  new_xp: number | null;
  created_at: string;
}

export interface RunResponse {
  status: 'success' | 'compile_error' | 'timeout' | 'runtime_error';
  stdout: string;
  stderr: string;
  duration_ms: number;
}

// Achievement types
export interface Achievement {
  id: string;
  slug: string;
  name: string;
  description: string;
  icon: string;
  xp_reward: number;
  condition_type: string;
  condition_value: number;
  earned_at: string | null;
}

// Leaderboard types
export interface LeaderboardEntry {
  rank: number;
  user_id: string;
  username: string;
  avatar_url: string | null;
  level: number;
  xp: number;
}

export interface LeaderboardResponse {
  entries: LeaderboardEntry[];
  user_rank: LeaderboardEntry | null;
}

// Forum types
export type ForumCategory = 'general' | 'task_help' | 'show_and_tell';

export interface ForumPostSummary {
  id: string;
  user_id: string;
  username: string;
  title: string;
  category: string;
  votes: number;
  views: number;
  reply_count: number;
  is_pinned: boolean;
  task_id: string | null;
  created_at: string;
}

export interface ForumPostListResponse {
  posts: ForumPostSummary[];
  page: number;
  per_page: number;
  total: number;
}

export interface ForumReply {
  id: string;
  user_id: string;
  username: string;
  body_md: string;
  votes: number;
  is_accepted: boolean;
  created_at: string;
}

export interface ForumPostDetail {
  id: string;
  user_id: string;
  username: string;
  title: string;
  body_md: string;
  category: string;
  votes: number;
  views: number;
  reply_count: number;
  is_pinned: boolean;
  task_id: string | null;
  created_at: string;
  updated_at: string;
  replies: ForumReply[];
}

// Article types
export interface ArticleSummary {
  id: string;
  author_id: string;
  author_username: string;
  title: string;
  cover_image_url: string | null;
  tags: string[];
  views: number;
  likes: number;
  created_at: string;
}

export interface ArticleListResponse {
  articles: ArticleSummary[];
  page: number;
  per_page: number;
  total: number;
}

export interface ArticleDetail {
  id: string;
  author_id: string;
  author_username: string;
  title: string;
  body_md: string;
  cover_image_url: string | null;
  tags: string[];
  is_published: boolean;
  views: number;
  likes: number;
  created_at: string;
  updated_at: string;
}

// Review types
export interface Review {
  id: string;
  user_id: string;
  username: string;
  rating: number;
  body: string;
  is_featured: boolean;
  created_at: string;
}

export interface ReviewListResponse {
  reviews: Review[];
}

// Dashboard types
export interface RecentSubmission {
  id: string;
  task_id: string;
  task_slug: string;
  task_title: string;
  status: 'passed' | 'failed' | 'error' | 'timeout';
  created_at: string;
}

export interface DashboardData {
  user: UserProfile;
  recent_submissions: RecentSubmission[];
  recent_achievements: Achievement[];
  active_quest: Quest | null;
  leaderboard_rank: number | null;
}
