import { Link, Navigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import {
  CheckCircle,
  XCircle,
  AlertCircle,
  Clock,
  Trophy,
  ArrowRight,
  Swords,
  FileText,
  Award,
  BarChart3,
} from 'lucide-react';
import { useDashboard } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import { Card, Skeleton, Badge, Button } from '@/components/ui';
import { XPBar, LevelBadge, StreakFlame, AchievementGrid } from '@/components/gamification';
import { staggerContainer, fadeInUp, useReducedMotion } from '@/lib/motion';
import type { RecentSubmission, Achievement, Quest } from '@/types';

function StatusBadge({ status }: { status: RecentSubmission['status'] }) {
  switch (status) {
    case 'passed':
      return (
        <Badge variant="success">
          <CheckCircle className="w-3 h-3" aria-hidden="true" />
          Passed
        </Badge>
      );
    case 'failed':
      return (
        <Badge variant="error">
          <XCircle className="w-3 h-3" aria-hidden="true" />
          Failed
        </Badge>
      );
    case 'error':
      return (
        <Badge variant="warning">
          <AlertCircle className="w-3 h-3" aria-hidden="true" />
          Error
        </Badge>
      );
    case 'timeout':
      return (
        <Badge variant="default">
          <Clock className="w-3 h-3" aria-hidden="true" />
          Timeout
        </Badge>
      );
  }
}

function DashboardSkeleton() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {/* Active Quest skeleton */}
      <Card className="md:col-span-2 lg:col-span-1">
        <Skeleton className="h-5 w-32 mb-4" />
        <div className="flex items-start gap-4">
          <Skeleton className="w-12 h-12 rounded-lg" />
          <div className="flex-1 space-y-2">
            <Skeleton className="h-4 w-3/4" />
            <Skeleton className="h-3 w-1/2" />
            <Skeleton className="h-2 w-full rounded-full mt-3" />
            <Skeleton className="h-10 w-28 rounded-lg mt-4" />
          </div>
        </div>
      </Card>

      {/* Recent Submissions skeleton */}
      <Card>
        <Skeleton className="h-5 w-40 mb-4" />
        <div className="space-y-3">
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i} className="flex items-center justify-between py-2">
              <div className="flex-1 space-y-1.5">
                <Skeleton className="h-4 w-3/4" />
                <Skeleton className="h-3 w-1/3" />
              </div>
              <Skeleton className="h-5 w-16 rounded-full" />
            </div>
          ))}
        </div>
      </Card>

      {/* Achievements skeleton */}
      <Card>
        <div className="flex items-center justify-between mb-4">
          <Skeleton className="h-5 w-28" />
          <Skeleton className="h-4 w-14" />
        </div>
        <div className="grid grid-cols-4 gap-3">
          {Array.from({ length: 8 }).map((_, i) => (
            <Skeleton key={i} className="w-full aspect-square rounded-lg" />
          ))}
        </div>
      </Card>

      {/* Leaderboard Rank skeleton */}
      <Card>
        <div className="flex items-center justify-between mb-4">
          <Skeleton className="h-5 w-28" />
          <Skeleton className="h-4 w-16" />
        </div>
        <div className="flex items-center gap-4">
          <Skeleton className="w-12 h-12 rounded-full" />
          <div className="space-y-2">
            <Skeleton className="h-7 w-16" />
            <Skeleton className="h-3 w-24" />
          </div>
        </div>
      </Card>
    </div>
  );
}

function RecentSubmissionsWidget({ submissions }: { submissions: RecentSubmission[] }) {
  if (submissions.length === 0) {
    return (
      <Card>
        <div className="flex items-center gap-2 mb-4">
          <FileText className="w-5 h-5 text-text-muted" aria-hidden="true" />
          <h3 className="font-display text-lg font-bold text-text-primary">Recent Submissions</h3>
        </div>
        <p className="text-sm text-text-muted">No submissions yet. Start solving tasks!</p>
      </Card>
    );
  }

  return (
    <Card>
      <div className="flex items-center gap-2 mb-4">
        <FileText className="w-5 h-5 text-text-muted" aria-hidden="true" />
        <h3 className="font-display text-lg font-bold text-text-primary">Recent Submissions</h3>
      </div>
      <div className="space-y-2">
        {submissions.slice(0, 5).map((sub) => (
          <Link
            key={sub.id}
            to={`/tasks/${sub.task_slug}`}
            className="flex items-center justify-between py-2 px-3 rounded-lg hover:bg-surface-overlay/50 transition-colors cursor-pointer"
          >
            <div className="flex-1 min-w-0">
              <p className="text-sm text-text-primary truncate font-body">
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
    </Card>
  );
}

function AchievementsWidget({ achievements }: { achievements: Achievement[] }) {
  return (
    <Card>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Award className="w-5 h-5 text-text-muted" aria-hidden="true" />
          <h3 className="font-display text-lg font-bold text-text-primary">Achievements</h3>
        </div>
        <Link
          to="/profile"
          className="text-xs text-primary hover:text-primary-400 transition-colors cursor-pointer"
        >
          View all
        </Link>
      </div>
      {achievements.length === 0 ? (
        <p className="text-sm text-text-muted">No achievements earned yet.</p>
      ) : (
        <AchievementGrid achievements={achievements.slice(0, 8)} />
      )}
    </Card>
  );
}

function LeaderboardRankWidget({ rank, xp }: { rank: number | null; xp: number }) {
  return (
    <Card>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <BarChart3 className="w-5 h-5 text-text-muted" aria-hidden="true" />
          <h3 className="font-display text-lg font-bold text-text-primary">Leaderboard</h3>
        </div>
        <Link
          to="/leaderboard"
          className="text-xs text-primary hover:text-primary-400 transition-colors cursor-pointer"
        >
          View full
        </Link>
      </div>
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center">
          <Trophy className="w-5 h-5 text-primary" aria-hidden="true" />
        </div>
        <div>
          <p className="text-2xl font-bold text-text-primary font-display">
            {rank !== null ? `#${rank}` : '—'}
          </p>
          <p className="text-xs text-text-muted">{xp.toLocaleString()} XP total</p>
        </div>
      </div>
    </Card>
  );
}

function ActiveQuestWidget({ quest }: { quest: Quest | null }) {
  if (!quest) {
    return (
      <Card>
        <div className="flex items-center gap-2 mb-4">
          <Swords className="w-5 h-5 text-text-muted" aria-hidden="true" />
          <h3 className="font-display text-lg font-bold text-text-primary">Active Quest</h3>
        </div>
        <p className="text-sm text-text-muted mb-4">No active quest. Start your journey!</p>
        <Button asChild>
          <Link to="/quests">
            Browse Quests
            <ArrowRight className="w-4 h-4" aria-hidden="true" />
          </Link>
        </Button>
      </Card>
    );
  }

  const progress = quest.user_progress;
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  return (
    <Card>
      <div className="flex items-center gap-2 mb-4">
        <Swords className="w-5 h-5 text-text-muted" aria-hidden="true" />
        <h3 className="font-display text-lg font-bold text-text-primary">Active Quest</h3>
      </div>
      <div className="flex items-start gap-4">
        <div className="w-12 h-12 rounded-lg bg-primary/10 border border-primary/30 flex items-center justify-center shrink-0">
          <Swords className="w-6 h-6 text-primary" aria-hidden="true" />
        </div>
        <div className="flex-1 min-w-0">
          <p className="text-sm font-medium text-text-primary mb-1 truncate">{quest.title}</p>
          <p className="text-xs text-text-muted mb-3">
            {progress?.tasks_completed ?? 0} / {progress?.tasks_total ?? quest.task_count} tasks
          </p>
          <div className="h-2 bg-slate-800 rounded-full overflow-hidden mb-4">
            <div
              className="h-full rounded-full bg-gradient-to-r from-primary to-warning transition-all duration-500"
              style={{ width: `${progressPercent}%` }}
            />
          </div>
          <Button asChild size="sm">
            <Link to={`/quests/${quest.slug}`}>
              Continue
              <ArrowRight className="w-4 h-4" aria-hidden="true" />
            </Link>
          </Button>
        </div>
      </div>
    </Card>
  );
}

export default function Dashboard() {
  const { isAuthenticated, user } = useAuthStore();
  const { data, isLoading } = useDashboard();
  const prefersReducedMotion = useReducedMotion();

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  const containerVariants = prefersReducedMotion ? {} : staggerContainer;
  const itemVariants = prefersReducedMotion ? {} : fadeInUp;

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[1280px] mx-auto">
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
              <DashboardSkeleton />
            ) : (
              <motion.div
                className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
                variants={containerVariants}
                initial="hidden"
                animate="visible"
              >
                {/* Active Quest */}
                <motion.div className="md:col-span-2 lg:col-span-1" variants={itemVariants}>
                  <ActiveQuestWidget quest={data?.active_quest ?? null} />
                </motion.div>

                {/* Recent Submissions */}
                <motion.div className="md:col-span-1" variants={itemVariants}>
                  <RecentSubmissionsWidget submissions={data?.recent_submissions ?? []} />
                </motion.div>

                {/* Achievements */}
                <motion.div className="md:col-span-1" variants={itemVariants}>
                  <AchievementsWidget achievements={data?.recent_achievements ?? []} />
                </motion.div>

                {/* Leaderboard Rank */}
                <motion.div className="md:col-span-1" variants={itemVariants}>
                  <LeaderboardRankWidget rank={data?.leaderboard_rank ?? null} xp={user?.xp ?? 0} />
                </motion.div>
              </motion.div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
