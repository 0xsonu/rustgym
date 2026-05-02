import { motion } from 'framer-motion';

interface XPBarProps {
  currentXP: number;
  level: number;
  /** XP required per level (default 1000) */
  xpPerLevel?: number;
}

export default function XPBar({ currentXP, level, xpPerLevel = 1000 }: XPBarProps) {
  const xpInCurrentLevel = currentXP - level * xpPerLevel;
  const progress = Math.min(Math.max((xpInCurrentLevel / xpPerLevel) * 100, 0), 100);

  return (
    <div className="w-full">
      <div className="flex items-center justify-between mb-1.5">
        <span className="text-xs font-bold text-text-muted uppercase tracking-wide">
          Level {level}
        </span>
        <span className="text-xs font-code text-text-secondary">
          {xpInCurrentLevel} / {xpPerLevel} XP
        </span>
      </div>
      <div className="h-3 bg-dark-700 rounded-full overflow-hidden">
        <motion.div
          className="h-full rounded-full bg-gradient-to-r from-primary to-amber"
          initial={{ width: 0 }}
          animate={{ width: `${progress}%` }}
          transition={{ duration: 1.5, ease: 'easeOut' }}
        />
      </div>
    </div>
  );
}
