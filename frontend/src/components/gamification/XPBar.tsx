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

  // Level milestone markers at 25%, 50%, 75%
  const milestones = [25, 50, 75];

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
      <div className="relative h-3 bg-slate-800 rounded-full overflow-hidden">
        {/* Milestone markers */}
        {milestones.map((milestone) => (
          <div
            key={milestone}
            className="absolute top-0 bottom-0 w-px bg-slate-700 z-10"
            style={{ left: `${milestone}%` }}
          />
        ))}

        {/* Gradient fill */}
        <motion.div
          className="h-full rounded-full bg-gradient-to-r from-primary-700 via-primary to-primary-400"
          initial={{ width: 0 }}
          animate={{ width: `${progress}%` }}
          transition={{ duration: 1.5, ease: 'easeOut' }}
        />

        {/* Percentage label */}
        {progress > 10 && (
          <motion.span
            className="absolute inset-0 flex items-center justify-center text-[10px] font-bold text-white/90"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.8, duration: 0.3 }}
          >
            {Math.round(progress)}%
          </motion.span>
        )}
      </div>
      {/* Level milestone labels */}
      <div className="flex justify-between mt-1 px-0.5">
        <span className="text-[10px] text-text-muted">Lv {level}</span>
        <span className="text-[10px] text-text-muted">Lv {level + 1}</span>
      </div>
    </div>
  );
}
