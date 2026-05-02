import { Link, Navigate } from 'react-router-dom';
import { CheckCircle, XCircle, AlertCircle, Clock, Trophy, ArrowRight } from 'lucide-react';
import { useDashboard } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import { XPBar, LevelBadge, StreakFlame } from '@/components/gamification';
import type { RecentSubmission, Achievement, Quest } from '@/types';

function StatusBadge({ status }: { status: RecentSubmission['status'] }) {
  switch (status) {
    case 'passed':
      return (
        <span className="flex items-center gap-1 text-xs font-medium text-green">
          <CheckCircle className="w-3.5 h-3.5" />
          Passed
        </span>
      );
    case 'failed':
      return (
        <span className="flex items-center gap-1 text-xs font-medium text-primary">
          <XCircle className="w-3.5 h-3.5" />
          Failed
        </span>
      );
    case 'error':
      return (
        <span className="flex items-center gap-1 text-xs font-medium text-amber">
          <AlertCircle className="w-3.5 h-3.5" />
          Error
        </span>
      );
    case 'timeout':
      return (
        <span className="flex items-center gap-1 text-xs font-medium text-text-muted">
          <Clock className="w-3.5 h-3.5" />
          Timeout
        </span>
      );
  }
}

function RecentSubmissionsWidget({ submissions }: { submissions: RecentSubmission[] }) {
  if (submissions.length === 0) {
    return (
      <div className="bg-dark-card border border-border rounded-2xl p-6">
        <h3 className="font-display text-lg font-bold text-text-primary mb-4">
          Recent Submissions
        </h3>
        <p className="text-sm text-text-muted">No submissions yet. Start solving tasks!</p>
      </div>
    );
  }

  return (
    <div className="bg-dark-card border border-border rounded-2xl p-6">
      <h3 className="font-display text-lg font-bold text-text-primary mb-4">Recent Submissions</h3>
      <div className="space-y-3">
        {submissions.slice(0, 5).map((sub) => (
          <Link
            key={sub.id}
            to={`/tasks/${sub.task_slug}`}
            className="flex items-center justify-between py-2 px-3 rounded-lg hover:bg-dark-700 transition-colors"
          >
            <div className="flex-1 min-w-0">
              <p className="text-sm text-text-primary truncate">
                {sub.task_title || sub.task_slug}
              </p>
              <p className="text-[11px] text-text-muted">
                {new Date(sub.created_at).toLocaleDateString()}
              </p>
            </div>
            <StatusBadge status={sub.status} />
          </Link>
        ))}
      </div>
    </div>
  );
}

function AchievementsWidget({ achievements }: { achievements: Achievement[] }) {
  return (
    <div className="bg-dark-card border border-border rounded-2xl p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-display text-lg font-bold text-text-primary">Achievements</h3>
        <Link to="#" className="text-xs text-primary hover:text-primary-light transition-colors">
          View all
        </Link>
      </div>
      {achievements.length === 0 ? (
        <p className="text-sm text-text-muted">No achievements earned yet.</p>
      ) : (
        <div className="grid grid-cols-6 gap-2">
          {achievements.slice(0, 6).map((ach) => (
            <div
              key={ach.id}
              className="w-10 h-10 rounded-lg bg-dark-700 border border-border-light flex items-center justify-center text-lg"
              title={`${ach.name} - ${ach.description}`}
            >
              {ach.icon || '🏆'}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function LeaderboardRankWidget({ rank, xp }: { rank: number | null; xp: number }) {
  return (
    <div className="bg-dark-card border border-border rounded-2xl p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-display text-lg font-bold text-text-primary">Leaderboard</h3>
        <Link
          to="/leaderboard"
          className="text-xs text-primary hover:text-primary-light transition-colors"
        >
          View full
        </Link>
      </div>
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center">
          <Trophy className="w-5 h-5 text-primary" />
        </div>
        <div>
          <p className="text-2xl font-bold text-text-primary font-display">
            {rank !== null ? `#${rank}` : '—'}
          </p>
          <p className="text-xs text-text-muted">{xp.toLocaleString()} XP total</p>
        </div>
      </div>
    </div>
  );
}

function ActiveQuestWidget({ quest }: { quest: Quest | null }) {
  if (!quest) {
    return (
      <div className="bg-dark-card border border-border rounded-2xl p-6">
        <h3 className="font-display text-lg font-bold text-text-primary mb-4">Active Quest</h3>
        <p className="text-sm text-text-muted mb-4">No active quest. Start your journey!</p>
        <Link
          to="/quests"
          className="inline-flex items-center gap-2 bg-primary text-white px-5 py-2.5 rounded-[10px] text-sm font-semibold hover:bg-primary-light transition-all hover:-translate-y-0.5"
        >
          Browse Quests
          <ArrowRight className="w-4 h-4" />
        </Link>
      </div>
    );
  }

  const progress = quest.user_progress;
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  return (
    <div className="bg-dark-card border border-border rounded-2xl p-6">
      <h3 className="font-display text-lg font-bold text-text-primary mb-4">Active Quest</h3>
      <div className="flex items-start gap-4">
        <span className="text-3xl">{quest.icon || '📦'}</span>
        <div className="flex-1">
          <p className="text-sm font-medium text-text-primary mb-1">{quest.title}</p>
          <p className="text-xs text-text-muted mb-3">
            {progress?.tasks_completed ?? 0} / {progress?.tasks_total ?? quest.task_count} tasks
          </p>
          <div className="h-2 bg-dark-700 rounded-full overflow-hidden mb-4">
            <div
              className="h-full rounded-full bg-gradient-to-r from-primary to-amber transition-all"
              style={{ width: `${progressPercent}%` }}
            />
          </div>
          <Link
            to={`/quests/${quest.slug}`}
            className="inline-flex items-center gap-2 bg-primary text-white px-5 py-2.5 rounded-[10px] text-sm font-semibold hover:bg-primary-light transition-all hover:-translate-y-0.5"
          >
            Continue
            <ArrowRight className="w-4 h-4" />
          </Link>
        </div>
      </div>
    </div>
  );
}

export default function Dashboard() {
  const { isAuthenticated, user } = useAuthStore();
  const { data, isLoading } = useDashboard();

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return (
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[1200px] mx-auto">
            {/* Greeting */}
            <div className="flex items-center gap-4 mb-8">
              <LevelBadge level={user?.level ?? 1} size="lg" />
              <div>
                <h1 className="font-display text-3xl md:text-4xl font-extrabold text-text-primary">
                  Welcome back,{' '}
                  <span className="text-primary">{user?.username ?? 'Rustacean'}</span>!
                </h1>
                <div className="flex items-center gap-4 mt-2">
                  <StreakFlame streak={user?.streak_days ?? 0} />
                </div>
              </div>
            </div>

            {/* XP Bar */}
            <div className="mb-10 max-w-lg">
              <XPBar currentXP={user?.xp ?? 0} level={user?.level ?? 1} />
            </div>

            {isLoading ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {Array.from({ length: 4 }).map((_, i) => (
                  <div
                    key={i}
                    className="bg-dark-card border border-border rounded-2xl p-6 animate-pulse h-[180px]"
                  />
                ))}
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {/* Active Quest */}
                <div className="md:col-span-2 lg:col-span-1">
                  <ActiveQuestWidget quest={data?.active_quest ?? null} />
                </div>

                {/* Recent Submissions */}
                <div className="md:col-span-1">
                  <RecentSubmissionsWidget submissions={data?.recent_submissions ?? []} />
                </div>

                {/* Achievements */}
                <div className="md:col-span-1">
                  <AchievementsWidget achievements={data?.recent_achievements ?? []} />
                </div>

                {/* Leaderboard Rank */}
                <div className="md:col-span-1">
                  <LeaderboardRankWidget rank={data?.leaderboard_rank ?? null} xp={user?.xp ?? 0} />
                </div>
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
