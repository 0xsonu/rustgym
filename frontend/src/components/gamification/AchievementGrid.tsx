import { useState } from 'react';
import type { Achievement } from '@/types';

interface AchievementGridProps {
  achievements: Achievement[];
}

function AchievementTile({ achievement }: { achievement: Achievement }) {
  const [showTooltip, setShowTooltip] = useState(false);
  const isUnlocked = !!achievement.earned_at;

  return (
    <div
      className="relative"
      onMouseEnter={() => setShowTooltip(true)}
      onMouseLeave={() => setShowTooltip(false)}
    >
      <div
        className={`w-16 h-16 rounded-xl flex items-center justify-center text-2xl border transition-all ${
          isUnlocked
            ? 'bg-dark-card border-border-light'
            : 'bg-dark-700 border-border grayscale opacity-50'
        }`}
      >
        {achievement.icon || '🏆'}
      </div>

      {showTooltip && (
        <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 z-50 w-48 bg-dark-800 border border-border rounded-lg p-3 shadow-[0_10px_40px_rgba(0,0,0,0.5)]">
          <p className="text-xs font-bold text-text-primary mb-1">{achievement.name}</p>
          <p className="text-[11px] text-text-secondary mb-1.5">{achievement.description}</p>
          <div className="flex items-center justify-between">
            <span className="text-[10px] font-code text-amber">+{achievement.xp_reward} XP</span>
            {isUnlocked && achievement.earned_at && (
              <span className="text-[10px] text-text-muted">
                {new Date(achievement.earned_at).toLocaleDateString()}
              </span>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export default function AchievementGrid({ achievements }: AchievementGridProps) {
  return (
    <div className="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 gap-3">
      {achievements.map((achievement) => (
        <AchievementTile key={achievement.id} achievement={achievement} />
      ))}
    </div>
  );
}
