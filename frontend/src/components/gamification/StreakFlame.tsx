import { motion } from 'framer-motion';

interface StreakFlameProps {
  streak: number;
}

export default function StreakFlame({ streak }: StreakFlameProps) {
  const isActive = streak > 0;

  return (
    <div className="flex items-center gap-1.5">
      <motion.span
        className="text-xl"
        animate={isActive ? { scale: [1, 1.15, 1] } : {}}
        transition={isActive ? { duration: 1.5, repeat: Infinity, ease: 'easeInOut' } : {}}
      >
        🔥
      </motion.span>
      <span
        className={`font-bold font-code text-sm ${isActive ? 'text-amber' : 'text-text-muted'}`}
      >
        {streak} day{streak !== 1 ? 's' : ''}
      </span>
    </div>
  );
}
