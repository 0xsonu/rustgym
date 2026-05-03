import { Trophy, Lock } from 'lucide-react';
import { Card } from '@/components/ui/Card';
import { Tooltip } from '@/components/ui/Tooltip';
import type { Achievement } from '@/types';

interface AchievementGridProps {
  achievements: Achievement[];
}

function AchievementTile({ achievement }: { achievement: Achievement }) {
  const isUnlocked = !!achievement.earned_at;

  const tooltipContent = isUnlocked
    ? `${achievement.name} — ${achievement.description} (+${achievement.xp_reward} XP)`
    : `${achievement.name} — ${achievement.description} (Locked)`;

  return (
    <Tooltip content={tooltipContent} side="top">
      <Card
        className={`w-16 h-16 !p-0 flex items-center justify-center transition-all ${
          isUnlocked
            ? 'border-warning/30 bg-surface-elevated'
            : 'border-border bg-slate-900 opacity-50'
        }`}
      >
        {isUnlocked ? (
          <Trophy size={24} className="text-warning" aria-hidden="true" />
        ) : (
          <Lock size={20} className="text-text-muted" aria-hidden="true" />
        )}
      </Card>
    </Tooltip>
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
