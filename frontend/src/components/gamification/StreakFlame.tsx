import { motion } from 'framer-motion';
import { Flame } from 'lucide-react';

interface StreakFlameProps {
  streak: number;
}

export default function StreakFlame({ streak }: StreakFlameProps) {
  const isActive = streak > 0;

  // Scale animation intensity based on streak length (capped at 1.2)
  const scaleIntensity = Math.min(1 + streak * 0.01, 1.2);

  return (
    <div className="flex items-center gap-1.5">
      <motion.div
        animate={isActive ? { scale: [1, scaleIntensity, 1] } : {}}
        transition={isActive ? { duration: 1.5, repeat: Infinity, ease: 'easeInOut' } : {}}
      >
        <Flame
          size={20}
          className={isActive ? 'text-warning fill-warning/50' : 'text-text-muted'}
          aria-hidden="true"
        />
      </motion.div>
      <span
        className={`font-bold font-code text-sm ${isActive ? 'text-warning' : 'text-text-muted'}`}
      >
        {streak} day{streak !== 1 ? 's' : ''}
      </span>
    </div>
  );
}
