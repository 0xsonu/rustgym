export interface AdminStats {
  total_users: number;
  submissions_today: number;
  pass_rate: number;
  popular_tasks: PopularTask[];
}

export interface PopularTask {
  id: string;
  slug: string;
  title: string;
  submission_count: number;
}

export interface AdminQuest {
  id: string;
  slug: string;
  title: string;
  description: string | null;
  order_index: number;
  icon: string | null;
  color: string | null;
  is_published: boolean;
  prerequisite_quest_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface AdminLevel {
  id: string;
  quest_id: string;
  slug: string;
  title: string;
  description: string | null;
  order_index: number;
  is_published: boolean;
  created_at: string;
  updated_at: string;
}

export interface AdminTask {
  id: string;
  level_id: string;
  slug: string;
  title: string;
  description_md: string;
  starter_code: string;
  solution_code: string;
  test_code: string;
  cargo_toml: string;
  difficulty: string;
  xp_reward: number;
  order_index: number;
  is_published: boolean;
  tags: string[];
  hint_md: string | null;
  syntest_rules: string[];
  created_at: string;
  updated_at: string;
}

export interface AdminAchievement {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  icon: string | null;
  xp_reward: number;
  condition_type: string;
  condition_value: number;
  condition_meta: unknown;
  created_at: string;
  updated_at: string;
}

export interface AdminUser {
  id: string;
  username: string;
  email: string;
  role: string;
  xp: number;
  level: number;
  streak_days: number;
  is_verified: boolean;
  is_banned: boolean;
  created_at: string;
}

export interface AdminUserListResponse {
  users: AdminUser[];
  page: number;
  per_page: number;
  total: number;
}

export interface AdminSubmission {
  id: string;
  user_id: string;
  task_id: string;
  status: string;
  duration_ms: number;
  memory_kb: number;
  xp_awarded: number;
  attempt_number: number;
  created_at: string;
}

export interface AdminSubmissionListResponse {
  submissions: AdminSubmission[];
  page: number;
  per_page: number;
  total: number;
}
