import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Edit2, Save, X, Trophy, Flame, Calendar } from 'lucide-react';
import { useProfile, useUpdateProfile, useAchievements } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';

export default function Profile() {
  const { username } = useParams<{ username: string }>();
  const { user: currentUser } = useAuthStore();
  const { data: profile, isLoading } = useProfile(username ?? '');
  const { data: achievements } = useAchievements();
  const updateProfile = useUpdateProfile();

  const [isEditing, setIsEditing] = useState(false);
  const [editBio, setEditBio] = useState('');
  const [editUsername, setEditUsername] = useState('');

  const isOwnProfile = currentUser?.username === username;

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
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto">
              <div className="bg-dark-card border border-border rounded-2xl p-8 animate-pulse h-64" />
            </div>
          </section>
        </main>
      </div>
    );
  }

  if (!profile) {
    return (
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto text-center">
              <p className="text-text-secondary">User not found.</p>
            </div>
          </section>
        </main>
      </div>
    );
  }

  const earnedAchievements = achievements?.filter((a) => a.earned_at !== null) ?? [];

  return (
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[800px] mx-auto">
            {/* Profile Header */}
            <div className="bg-dark-card border border-border rounded-2xl p-8 mb-6">
              <div className="flex items-start gap-6">
                {/* Avatar */}
                <div className="w-20 h-20 rounded-full bg-primary/20 border-2 border-primary/40 flex items-center justify-center shrink-0">
                  {profile.avatar_url ? (
                    <img
                      src={profile.avatar_url}
                      alt={profile.username}
                      className="w-full h-full rounded-full object-cover"
                    />
                  ) : (
                    <span className="text-2xl font-bold text-primary">
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
                        className="bg-dark-700 border border-border rounded-lg px-3 py-1.5 text-lg font-bold text-text-primary focus:outline-none focus:border-primary"
                      />
                    ) : (
                      <h1 className="font-display text-2xl font-bold text-text-primary">
                        {profile.username}
                      </h1>
                    )}
                    <span className="inline-flex items-center justify-center w-7 h-7 rounded-full bg-amber/20 border border-amber/40 text-xs font-bold text-amber">
                      {profile.level}
                    </span>
                  </div>

                  {isEditing ? (
                    <textarea
                      value={editBio}
                      onChange={(e) => setEditBio(e.target.value)}
                      placeholder="Write a short bio..."
                      rows={2}
                      className="w-full bg-dark-700 border border-border rounded-lg px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-primary resize-none mb-3"
                    />
                  ) : (
                    <p className="text-sm text-text-secondary mb-3">
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
                          className="flex items-center gap-1.5 bg-primary text-white px-3 py-1.5 rounded-lg text-xs font-semibold transition-all hover:bg-primary-light disabled:opacity-50"
                        >
                          <Save className="w-3.5 h-3.5" />
                          Save
                        </button>
                        <button
                          onClick={() => setIsEditing(false)}
                          className="flex items-center gap-1.5 bg-dark-700 text-text-secondary px-3 py-1.5 rounded-lg text-xs font-medium transition-all hover:text-text-primary"
                        >
                          <X className="w-3.5 h-3.5" />
                          Cancel
                        </button>
                      </div>
                    ) : (
                      <button
                        onClick={handleStartEdit}
                        className="flex items-center gap-1.5 bg-dark-700 text-text-secondary px-3 py-1.5 rounded-lg text-xs font-medium transition-all hover:text-text-primary hover:bg-dark-600"
                      >
                        <Edit2 className="w-3.5 h-3.5" />
                        Edit
                      </button>
                    )}
                  </div>
                )}
              </div>
            </div>

            {/* Stats */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
              <div className="bg-dark-card border border-border rounded-xl p-4 text-center">
                <p className="text-2xl font-bold text-primary font-code">{profile.xp}</p>
                <p className="text-xs text-text-muted mt-1">Total XP</p>
              </div>
              <div className="bg-dark-card border border-border rounded-xl p-4 text-center">
                <p className="text-2xl font-bold text-amber font-code">{profile.level}</p>
                <p className="text-xs text-text-muted mt-1">Level</p>
              </div>
              <div className="bg-dark-card border border-border rounded-xl p-4 text-center">
                <div className="flex items-center justify-center gap-1">
                  <Flame className="w-5 h-5 text-orange-400" />
                  <p className="text-2xl font-bold text-orange-400 font-code">
                    {profile.streak_days}
                  </p>
                </div>
                <p className="text-xs text-text-muted mt-1">Day Streak</p>
              </div>
              <div className="bg-dark-card border border-border rounded-xl p-4 text-center">
                <div className="flex items-center justify-center gap-1">
                  <Trophy className="w-5 h-5 text-yellow-400" />
                  <p className="text-2xl font-bold text-yellow-400 font-code">
                    {earnedAchievements.length}
                  </p>
                </div>
                <p className="text-xs text-text-muted mt-1">Achievements</p>
              </div>
            </div>

            {/* Achievements Grid */}
            {earnedAchievements.length > 0 && (
              <div className="bg-dark-card border border-border rounded-2xl p-6">
                <h2 className="text-sm font-bold text-text-primary uppercase tracking-wide mb-4">
                  Achievements
                </h2>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                  {earnedAchievements.map((achievement) => (
                    <div
                      key={achievement.id}
                      className="bg-dark-700 border border-border rounded-xl p-3 text-center"
                    >
                      <span className="text-2xl block mb-1">{achievement.icon}</span>
                      <p className="text-xs font-medium text-text-primary truncate">
                        {achievement.name}
                      </p>
                      <p className="text-[10px] text-text-muted mt-0.5">
                        +{achievement.xp_reward} XP
                      </p>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
