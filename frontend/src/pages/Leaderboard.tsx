import { useState } from 'react';
import { motion } from 'framer-motion';
import { Medal, Crown, Award, Trophy } from 'lucide-react';
import { useLeaderboard } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton } from '@/components/ui';
import { LevelBadge } from '@/components/gamification';
import { fadeInUp, staggerContainer } from '@/lib/motion';
import type { LeaderboardEntry } from '@/types';

type Period = 'alltime' | 'weekly' | 'monthly';

const tabs: { label: string; value: Period }[] = [
  { label: 'All-Time', value: 'alltime' },
  { label: 'Weekly', value: 'weekly' },
  { label: 'Monthly', value: 'monthly' },
];

function UserAvatar({
  username,
  avatarUrl,
  size = 'md',
}: {
  username: string;
  avatarUrl: string | null;
  size?: 'sm' | 'md' | 'lg';
}) {
  const sizeClasses = { sm: 'w-8 h-8', md: 'w-10 h-10', lg: 'w-14 h-14' };
  const textClasses = { sm: 'text-[10px]', md: 'text-xs', lg: 'text-base' };

  if (avatarUrl) {
    return (
      <img
        src={avatarUrl}
        alt={username}
        className={`${sizeClasses[size]} rounded-full object-cover`}
      />
    );
  }
  const initials = username.slice(0, 2).toUpperCase();
  return (
    <div
      className={`${sizeClasses[size]} rounded-full bg-primary/20 border border-primary/40 flex items-center justify-center`}
    >
      <span className={`${textClasses[size]} font-bold text-primary`}>{initials}</span>
    </div>
  );
}

function MedalIcon({ rank }: { rank: number }) {
  switch (rank) {
    case 1:
      return <Crown className="w-6 h-6 text-yellow-400" aria-hidden="true" />;
    case 2:
      return <Medal className="w-6 h-6 text-slate-300" aria-hidden="true" />;
    case 3:
      return <Award className="w-6 h-6 text-amber-600" aria-hidden="true" />;
    default:
      return null;
  }
}

function TopThreeCards({
  entries,
  currentUserId,
}: {
  entries: LeaderboardEntry[];
  currentUserId?: string;
}) {
  if (entries.length === 0) return null;

  const top3 = entries.slice(0, 3);
  const bgColors = [
    'bg-yellow-500/5 border-yellow-500/30',
    'bg-slate-300/5 border-slate-400/30',
    'bg-amber-600/5 border-amber-700/30',
  ];

  return (
    <motion.div
      className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-10"
      variants={staggerContainer}
      initial="hidden"
      animate="visible"
    >
      {top3.map((entry, i) => {
        const isCurrentUser = entry.user_id === currentUserId;
        return (
          <motion.div
            key={entry.user_id}
            variants={fadeInUp}
            className={`relative rounded-xl border p-6 text-center transition-all duration-200 ${bgColors[i]} ${
              isCurrentUser ? 'ring-2 ring-primary/50' : ''
            }`}
          >
            <div className="flex justify-center mb-3">
              <MedalIcon rank={entry.rank} />
            </div>
            <div className="flex justify-center mb-3">
              <UserAvatar username={entry.username} avatarUrl={entry.avatar_url} size="lg" />
            </div>
            <p className="font-display text-sm font-semibold text-text-primary truncate mb-1">
              {entry.username}
            </p>
            <div className="flex justify-center mb-2">
              <LevelBadge level={entry.level} size="sm" />
            </div>
            <p className="text-lg font-bold font-code text-amber tabular-nums">
              {entry.xp.toLocaleString()} XP
            </p>
            <Badge variant={i === 0 ? 'warning' : 'default'} className="mt-3">
              #{entry.rank}
            </Badge>
          </motion.div>
        );
      })}
    </motion.div>
  );
}

function LeaderboardTable({
  entries,
  currentUserId,
}: {
  entries: LeaderboardEntry[];
  currentUserId?: string;
}) {
  const tableEntries = entries.slice(3);

  if (tableEntries.length === 0) return null;

  return (
    <>
      {/* Desktop table view */}
      <div className="hidden md:block">
        <Card className="overflow-hidden p-0">
          <table className="w-full">
            <thead>
              <tr className="border-b border-border">
                <th className="text-left text-xs font-bold text-text-muted uppercase tracking-wide px-6 py-3">
                  Rank
                </th>
                <th className="text-left text-xs font-bold text-text-muted uppercase tracking-wide px-6 py-3">
                  User
                </th>
                <th className="text-left text-xs font-bold text-text-muted uppercase tracking-wide px-6 py-3">
                  Level
                </th>
                <th className="text-right text-xs font-bold text-text-muted uppercase tracking-wide px-6 py-3">
                  XP
                </th>
              </tr>
            </thead>
            <tbody>
              {tableEntries.map((entry, i) => {
                const isCurrentUser = entry.user_id === currentUserId;
                return (
                  <tr
                    key={entry.user_id}
                    className={`border-b border-border last:border-b-0 transition-colors ${
                      isCurrentUser
                        ? 'bg-primary/5 border-l-2 border-l-primary'
                        : i % 2 === 0
                          ? 'bg-surface-elevated'
                          : 'bg-surface-base/50'
                    }`}
                  >
                    <td className="px-6 py-3">
                      <span className="text-sm font-bold text-text-secondary">#{entry.rank}</span>
                    </td>
                    <td className="px-6 py-3">
                      <div className="flex items-center gap-3">
                        <UserAvatar
                          username={entry.username}
                          avatarUrl={entry.avatar_url}
                          size="sm"
                        />
                        <span
                          className={`text-sm font-medium ${isCurrentUser ? 'text-primary' : 'text-text-primary'}`}
                        >
                          {entry.username}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-3">
                      <LevelBadge level={entry.level} size="sm" />
                    </td>
                    <td className="px-6 py-3 text-right">
                      <span className="text-sm font-code text-amber tabular-nums">
                        {entry.xp.toLocaleString()}
                      </span>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </Card>
      </div>

      {/* Mobile card-based list */}
      <div className="md:hidden space-y-3">
        {tableEntries.map((entry) => {
          const isCurrentUser = entry.user_id === currentUserId;
          return (
            <Card
              key={entry.user_id}
              className={`p-4 ${isCurrentUser ? 'border-primary/50 bg-primary/5' : ''}`}
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <span className="text-sm font-bold text-text-muted w-8">#{entry.rank}</span>
                  <UserAvatar username={entry.username} avatarUrl={entry.avatar_url} size="sm" />
                  <div>
                    <p
                      className={`text-sm font-medium ${isCurrentUser ? 'text-primary' : 'text-text-primary'}`}
                    >
                      {entry.username}
                    </p>
                    <div className="flex items-center gap-2 mt-0.5">
                      <LevelBadge level={entry.level} size="sm" />
                    </div>
                  </div>
                </div>
                <span className="text-sm font-code text-amber tabular-nums">
                  {entry.xp.toLocaleString()}
                </span>
              </div>
            </Card>
          );
        })}
      </div>
    </>
  );
}

function CurrentUserRank({ userRank }: { userRank: LeaderboardEntry }) {
  return (
    <Card className="mt-6 border-primary/30 bg-primary/5">
      <div className="flex items-center gap-2 mb-3">
        <Trophy className="w-4 h-4 text-primary" aria-hidden="true" />
        <p className="text-xs text-text-muted uppercase tracking-wide font-bold">Your Rank</p>
      </div>
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <span className="text-lg font-bold text-primary">#{userRank.rank}</span>
          <UserAvatar username={userRank.username} avatarUrl={userRank.avatar_url} />
          <div>
            <span className="text-sm font-medium text-text-primary">{userRank.username}</span>
            <div className="mt-1">
              <LevelBadge level={userRank.level} size="sm" />
            </div>
          </div>
        </div>
        <Badge variant="primary" className="text-sm">
          {userRank.xp.toLocaleString()} XP
        </Badge>
      </div>
    </Card>
  );
}

function LeaderboardSkeleton() {
  return (
    <div className="space-y-6">
      {/* Top 3 skeleton */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        {Array.from({ length: 3 }).map((_, i) => (
          <Skeleton key={i} className="h-48 rounded-xl" />
        ))}
      </div>
      {/* Table skeleton */}
      <div className="space-y-2">
        {Array.from({ length: 5 }).map((_, i) => (
          <Skeleton key={i} className="h-14 rounded-lg" />
        ))}
      </div>
    </div>
  );
}

export default function Leaderboard() {
  const [period, setPeriod] = useState<Period>('alltime');
  const { data, isLoading } = useLeaderboard(period);
  const { user } = useAuthStore();

  const entries = data?.entries ?? [];
  const userRank = data?.user_rank ?? null;

  const userInTop = user ? entries.some((e) => e.user_id === user.id) : false;

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[900px] mx-auto">
            <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-3">RANKINGS</p>
            <h1 className="font-display text-4xl md:text-5xl font-extrabold text-text-primary mb-8">
              Leader<span className="text-primary">board</span>
            </h1>

            {/* Period Tabs */}
            <div className="flex gap-1 bg-surface-elevated border border-border rounded-xl p-1 mb-10 w-fit">
              {tabs.map((tab) => (
                <button
                  key={tab.value}
                  onClick={() => setPeriod(tab.value)}
                  className={`px-5 py-2 rounded-lg text-sm font-medium transition-all cursor-pointer ${
                    period === tab.value
                      ? 'bg-primary text-white'
                      : 'text-text-secondary hover:text-text-primary hover:bg-surface-overlay'
                  }`}
                >
                  {tab.label}
                </button>
              ))}
            </div>

            {isLoading ? (
              <LeaderboardSkeleton />
            ) : (
              <>
                {/* Top 3 Cards with SVG medals */}
                <TopThreeCards entries={entries} currentUserId={user?.id} />

                {/* Table for ranks 4+ with alternating rows */}
                <LeaderboardTable entries={entries} currentUserId={user?.id} />

                {/* Current user rank if not in top */}
                {!userInTop && userRank && <CurrentUserRank userRank={userRank} />}
              </>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
