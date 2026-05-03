import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Edit2, Save, X, Trophy, Flame, Calendar, Zap, Target, CheckCircle2 } from 'lucide-react';
import { useProfile, useUpdateProfile, useAchievements } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import { Card } from '@/components/ui';
import { Tooltip } from '@/components/ui';
import { Skeleton } from '@/components/ui';
import Navbar from '@/components/layout/Navbar';

export default function Profile() {
  const { username } = useParams<{ username: string }>();
  const { user: currentUser } = useAuthStore();
  const isOwnProfile = currentUser?.username === username;

  const { data: profile, isLoading } = useProfile(username ?? '');
  const { data: achievements } = useAchievements(isOwnProfile);
  const updateProfile = useUpdateProfile();

  const [isEditing, setIsEditing] = useState(false);
  const [editBio, setEditBio] = useState('');
  const [editUsername, setEditUsername] = useState('');

  const handleStartEdit = () => {
    setEditBio(profile?.bio ?? '');
    setEditUsername(profile?.username ?? '');
    setIsEditing(true);
  };

  const handleSave = () => {
    const data: { username?: string; bio?: string } = {};
    if (editBio !== (profile?.bio ?? '')) data.bio = editBio;
    if (editUsername !== profile?.username) data.username = editUsername;

    if (Object.keys(data).length > 0) {
      updateProfile.mutate(data, {
        onSuccess: () => setIsEditing(false),
      });
    } else {
      setIsEditing(false);
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto space-y-6">
              <Skeleton className="h-64 w-full rounded-2xl" />
              <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
                {Array.from({ length: 5 }).map((_, i) => (
                  <Skeleton key={i} className="h-24 w-full rounded-xl" />
                ))}
              </div>
              <Skeleton className="h-48 w-full rounded-2xl" />
            </div>
          </section>
        </main>
      </div>
    );
  }

  if (!profile) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto text-center">
              <p className="text-text-secondary font-body">User not found.</p>
            </div>
          </section>
        </main>
      </div>
    );
  }

  const earnedAchievements = achievements?.filter((a) => a.earned_at !== null) ?? [];

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[800px] mx-auto">
            {/* Profile Header */}
            <Card className="p-8 mb-6">
              <div className="flex flex-col sm:flex-row items-start gap-6">
                {/* Avatar */}
                <div className="w-20 h-20 rounded-full bg-primary/20 border-2 border-primary/40 flex items-center justify-center shrink-0">
                  {profile.avatar_url ? (
                    <img
                      src={profile.avatar_url}
                      alt={profile.username}
                      className="w-full h-full rounded-full object-cover"
                    />
                  ) : (
                    <span className="text-2xl font-bold text-primary font-display">
                      {profile.username.slice(0, 2).toUpperCase()}
                    </span>
                  )}
                </div>

                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3 mb-2">
                    {isEditing ? (
                      <input
                        type="text"
                        value={editUsername}
                        onChange={(e) => setEditUsername(e.target.value)}
                        className="bg-surface-base border border-border rounded-lg px-3 py-1.5 text-lg font-bold text-text-primary font-display focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary"
                      />
                    ) : (
                      <h1 className="font-display text-2xl font-bold text-text-primary">
                        {profile.username}
                      </h1>
                    )}
                    <span className="inline-flex items-center justify-center w-7 h-7 rounded-full bg-warning/20 border border-warning/40 text-xs font-bold text-warning">
                      {profile.level}
                    </span>
                  </div>

                  {isEditing ? (
                    <textarea
                      value={editBio}
                      onChange={(e) => setEditBio(e.target.value)}
                      placeholder="Write a short bio..."
                      rows={2}
                      className="w-full bg-surface-base border border-border rounded-lg px-3 py-2 text-sm text-text-primary font-body placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary resize-none mb-3"
                    />
                  ) : (
                    <p className="text-sm text-text-secondary font-body mb-3">
                      {profile.bio || 'No bio yet.'}
                    </p>
                  )}

                  <div className="flex items-center gap-2 text-xs text-text-muted">
                    <Calendar className="w-3.5 h-3.5" />
                    Joined {new Date(profile.created_at).toLocaleDateString()}
                  </div>
                </div>

                {/* Edit Button */}
                {isOwnProfile && (
                  <div className="shrink-0">
                    {isEditing ? (
                      <div className="flex gap-2">
                        <button
                          onClick={handleSave}
                          disabled={updateProfile.isPending}
                          className="flex items-center gap-1.5 bg-primary text-white px-3 py-1.5 rounded-lg text-xs font-semibold transition-all duration-200 hover:bg-primary-600 active:scale-[0.98] disabled:opacity-50 cursor-pointer"
                        >
                          <Save className="w-3.5 h-3.5" />
                          Save
                        </button>
                        <button
                          onClick={() => setIsEditing(false)}
                          className="flex items-center gap-1.5 bg-surface-overlay text-text-secondary px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-200 hover:text-text-primary cursor-pointer"
                        >
                          <X className="w-3.5 h-3.5" />
                          Cancel
                        </button>
                      </div>
                    ) : (
                      <button
                        onClick={handleStartEdit}
                        className="flex items-center gap-1.5 bg-surface-overlay text-text-secondary px-3 py-1.5 rounded-lg text-xs font-medium transition-all duration-200 hover:text-text-primary hover:bg-slate-700 cursor-pointer"
                      >
                        <Edit2 className="w-3.5 h-3.5" />
                        Edit
                      </button>
                    )}
                  </div>
                )}
              </div>
            </Card>

            {/* Stats Metric Cards */}
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4 mb-6">
              <Card className="p-4 text-center">
                <Zap className="w-5 h-5 text-primary mx-auto mb-2" />
                <p className="text-2xl font-bold text-primary font-code tabular-nums">
                  {profile.xp}
                </p>
                <p className="text-xs text-text-muted mt-1 font-body">Total XP</p>
              </Card>
              <Card className="p-4 text-center">
                <Target className="w-5 h-5 text-warning mx-auto mb-2" />
                <p className="text-2xl font-bold text-warning font-code tabular-nums">
                  {profile.level}
                </p>
                <p className="text-xs text-text-muted mt-1 font-body">Level</p>
              </Card>
              <Card className="p-4 text-center">
                <Flame className="w-5 h-5 text-orange-400 mx-auto mb-2" />
                <p className="text-2xl font-bold text-orange-400 font-code tabular-nums">
                  {profile.streak_days}
                </p>
                <p className="text-xs text-text-muted mt-1 font-body">Day Streak</p>
              </Card>
              <Card className="p-4 text-center">
                <CheckCircle2 className="w-5 h-5 text-success mx-auto mb-2" />
                <p className="text-2xl font-bold text-success font-code tabular-nums">
                  {earnedAchievements.length}
                </p>
                <p className="text-xs text-text-muted mt-1 font-body">Completed</p>
              </Card>
              <Card className="p-4 text-center">
                <Trophy className="w-5 h-5 text-info mx-auto mb-2" />
                <p className="text-2xl font-bold text-info font-code tabular-nums">
                  {earnedAchievements.length}
                </p>
                <p className="text-xs text-text-muted mt-1 font-body">Achievements</p>
              </Card>
            </div>

            {/* Achievements Grid */}
            {earnedAchievements.length > 0 && (
              <Card className="p-6 mb-6">
                <h2 className="font-display text-sm font-bold text-text-primary uppercase tracking-wide mb-4">
                  Achievements
                </h2>
                <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-3 auto-rows-[112px]">
                  {earnedAchievements.map((achievement) => (
                    <Tooltip
                      key={achievement.id}
                      content={`${achievement.name}: ${achievement.description ?? ''} (+${achievement.xp_reward} XP)`}
                      className="flex"
                    >
                      <div className="flex-1 bg-surface-base border border-border rounded-xl p-3 flex flex-col items-center justify-center text-center cursor-default">
                        <div className="w-10 h-10 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center shrink-0">
                          <Trophy className="w-5 h-5 text-primary" />
                        </div>
                        <p className="text-[11px] font-medium text-text-primary line-clamp-2 leading-tight mt-2 max-w-full px-1">
                          {achievement.name}
                        </p>
                      </div>
                    </Tooltip>
                  ))}
                </div>
              </Card>
            )}

            {/* Recent Activity Feed */}
            <Card className="p-6">
              <h2 className="font-display text-sm font-bold text-text-primary uppercase tracking-wide mb-4">
                Recent Activity
              </h2>
              <div className="space-y-3">
                {earnedAchievements.length > 0 ? (
                  earnedAchievements.slice(0, 5).map((achievement) => (
                    <div
                      key={achievement.id}
                      className="flex items-center gap-3 p-3 bg-surface-base rounded-lg border border-border"
                    >
                      <div className="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center shrink-0">
                        <Trophy className="w-4 h-4 text-primary" />
                      </div>
                      <div className="flex-1 min-w-0">
                        <p className="text-sm text-text-primary font-body">
                          Earned <span className="font-semibold">{achievement.name}</span>
                        </p>
                        <p className="text-xs text-text-muted">+{achievement.xp_reward} XP</p>
                      </div>
                      {achievement.earned_at && (
                        <span className="text-xs text-text-muted shrink-0">
                          {new Date(achievement.earned_at).toLocaleDateString()}
                        </span>
                      )}
                    </div>
                  ))
                ) : (
                  <p className="text-sm text-text-muted text-center py-4">No recent activity</p>
                )}
              </div>
            </Card>
          </div>
        </section>
      </main>
    </div>
  );
}
