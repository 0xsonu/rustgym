import { useState } from 'react';
import { motion } from 'framer-motion';
import { useLeaderboard } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import { LevelBadge } from '@/components/gamification';
import type { LeaderboardEntry } from '@/types';

type Period = 'alltime' | 'weekly' | 'monthly';

const tabs: { label: string; value: Period }[] = [
  { label: 'All-Time', value: 'alltime' },
  { label: 'Weekly', value: 'weekly' },
  { label: 'Monthly', value: 'monthly' },
];

function UserAvatar({ username, avatarUrl }: { username: string; avatarUrl: string | null }) {
  if (avatarUrl) {
    return <img src={avatarUrl} alt={username} className="w-10 h-10 rounded-full object-cover" />;
  }
  const initials = username.slice(0, 2).toUpperCase();
  return (
    <div className="w-10 h-10 rounded-full bg-primary/20 border border-primary/40 flex items-center justify-center">
      <span className="text-xs font-bold text-primary">{initials}</span>
    </div>
  );
}

function Podium({ entries }: { entries: LeaderboardEntry[] }) {
  if (entries.length < 3) return null;

  const first = entries[0]!;
  const second = entries[1]!;
  const third = entries[2]!;
  const podiumOrder = [second, first, third];
  const heights = ['h-28', 'h-36', 'h-24'];
  const medals = ['🥈', '🥇', '🥉'];
  const delays = [0.2, 0, 0.4];

  return (
    <div className="flex items-end justify-center gap-4 mb-12">
      {podiumOrder.map((entry, i) => (
        <motion.div
          key={entry.user_id}
          className="flex flex-col items-center"
          initial={{ opacity: 0, y: 40 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: delays[i], duration: 0.5, ease: 'easeOut' }}
        >
          <UserAvatar username={entry.username} avatarUrl={entry.avatar_url} />
          <p className="text-sm font-medium text-text-primary mt-2 truncate max-w-[100px]">
            {entry.username}
          </p>
          <p className="text-xs font-code text-amber">{entry.xp.toLocaleString()} XP</p>
          <div className="mt-2 text-2xl">{medals[i]}</div>
          <div
            className={`${heights[i]} w-20 bg-dark-card border border-border rounded-t-lg flex items-center justify-center mt-2`}
          >
            <span className="font-display text-2xl font-bold text-text-primary">#{entry.rank}</span>
          </div>
        </motion.div>
      ))}
    </div>
  );
}

function LeaderboardTable({ entries }: { entries: LeaderboardEntry[] }) {
  // Show entries from rank 4 onwards
  const tableEntries = entries.filter((e) => e.rank > 3);

  if (tableEntries.length === 0) return null;

  return (
    <div className="bg-dark-card border border-border rounded-2xl overflow-hidden">
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
          {tableEntries.map((entry) => (
            <tr
              key={entry.user_id}
              className="border-b border-border last:border-b-0 hover:bg-dark-700 transition-colors"
            >
              <td className="px-6 py-3">
                <span className="text-sm font-bold text-text-secondary">#{entry.rank}</span>
              </td>
              <td className="px-6 py-3">
                <div className="flex items-center gap-3">
                  <UserAvatar username={entry.username} avatarUrl={entry.avatar_url} />
                  <span className="text-sm font-medium text-text-primary">{entry.username}</span>
                </div>
              </td>
              <td className="px-6 py-3">
                <LevelBadge level={entry.level} size="sm" />
              </td>
              <td className="px-6 py-3 text-right">
                <span className="text-sm font-code text-amber">{entry.xp.toLocaleString()}</span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function CurrentUserRank({ userRank }: { userRank: LeaderboardEntry }) {
  return (
    <div className="mt-6 bg-dark-card border border-primary/30 rounded-2xl p-4">
      <p className="text-xs text-text-muted mb-2 uppercase tracking-wide font-bold">Your Rank</p>
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <span className="text-lg font-bold text-primary">#{userRank.rank}</span>
          <UserAvatar username={userRank.username} avatarUrl={userRank.avatar_url} />
          <span className="text-sm font-medium text-text-primary">{userRank.username}</span>
          <LevelBadge level={userRank.level} size="sm" />
        </div>
        <span className="text-sm font-code text-amber">{userRank.xp.toLocaleString()} XP</span>
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

  // Check if current user is in the top entries
  const userInTop = user ? entries.some((e) => e.user_id === user.id) : false;

  return (
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[900px] mx-auto">
            <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-3">RANKINGS</p>
            <h1 className="font-display text-4xl md:text-5xl font-extrabold text-text-primary mb-8">
              Leader<span className="text-primary">board</span>
            </h1>

            {/* Tabs */}
            <div className="flex gap-1 bg-dark-card border border-border rounded-xl p-1 mb-10 w-fit">
              {tabs.map((tab) => (
                <button
                  key={tab.value}
                  onClick={() => setPeriod(tab.value)}
                  className={`px-5 py-2 rounded-lg text-sm font-medium transition-all ${
                    period === tab.value
                      ? 'bg-primary text-white'
                      : 'text-text-secondary hover:text-text-primary hover:bg-dark-700'
                  }`}
                >
                  {tab.label}
                </button>
              ))}
            </div>

            {isLoading ? (
              <div className="space-y-4">
                {Array.from({ length: 5 }).map((_, i) => (
                  <div
                    key={i}
                    className="bg-dark-card border border-border rounded-xl p-4 animate-pulse h-16"
                  />
                ))}
              </div>
            ) : (
              <>
                {/* Podium for top 3 */}
                <Podium entries={entries} />

                {/* Table for ranks 4+ */}
                <LeaderboardTable entries={entries} />

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
