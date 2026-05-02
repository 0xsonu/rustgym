import { useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import type { Achievement } from '@/types';

interface AchievementModalProps {
  achievement: Achievement | null;
  onDismiss: () => void;
}

const PARTICLE_COLORS = ['#CE422B', '#E8913A', '#4ADE80', '#60A5FA', '#F0EDE8'];

function generateParticles() {
  return Array.from({ length: 20 }).map((_, i) => ({
    left: `${(i * 5 + ((i * 37) % 100)) % 100}%`,
    color: PARTICLE_COLORS[i % 5],
    rotate: (i * 137) % 720,
    x: ((i * 47) % 200) - 100,
    duration: 2 + (i % 4) * 0.5,
    delay: (i % 10) * 0.05,
  }));
}

export default function AchievementModal({ achievement, onDismiss }: AchievementModalProps) {
  const particles = useMemo(() => generateParticles(), []);

  return (
    <AnimatePresence>
      {achievement && (
        <motion.div
          className="fixed inset-0 z-[1000] flex items-center justify-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.3 }}
        >
          {/* Backdrop */}
          <div className="absolute inset-0 bg-dark-950/80 backdrop-blur-sm" onClick={onDismiss} />

          {/* Confetti particles */}
          <div className="absolute inset-0 overflow-hidden pointer-events-none">
            {particles.map((p, i) => (
              <motion.div
                key={i}
                className="absolute w-2 h-2 rounded-full"
                style={{
                  left: p.left,
                  backgroundColor: p.color,
                }}
                initial={{ top: '-5%', opacity: 1 }}
                animate={{
                  top: '110%',
                  opacity: 0,
                  rotate: p.rotate,
                  x: p.x,
                }}
                transition={{
                  duration: p.duration,
                  delay: p.delay,
                  ease: 'easeOut',
                }}
              />
            ))}
          </div>

          {/* Modal content */}
          <motion.div
            className="relative bg-dark-card border border-border-light rounded-2xl p-10 text-center max-w-sm mx-4"
            initial={{ scale: 0.5, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.8, opacity: 0 }}
            transition={{ type: 'spring', damping: 15, stiffness: 200 }}
          >
            <motion.div
              className="text-6xl mb-4"
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={{ delay: 0.2, type: 'spring', damping: 10 }}
            >
              {achievement.icon || '🏆'}
            </motion.div>

            <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-2">
              Achievement Unlocked!
            </p>

            <h2 className="font-display text-2xl font-bold text-text-primary mb-2">
              {achievement.name}
            </h2>

            <p className="text-sm text-text-secondary mb-4">{achievement.description}</p>

            <div className="inline-block bg-primary/10 border border-primary/30 text-primary-light font-code text-sm font-bold px-4 py-1.5 rounded-full mb-6">
              +{achievement.xp_reward} XP
            </div>

            <button
              onClick={onDismiss}
              className="block w-full bg-primary text-white px-7 py-3 rounded-[10px] font-semibold hover:bg-primary-light transition-all hover:-translate-y-0.5"
            >
              Awesome!
            </button>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
