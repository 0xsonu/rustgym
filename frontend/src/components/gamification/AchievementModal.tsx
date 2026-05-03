import { motion } from 'framer-motion';
import { Trophy, Sparkles } from 'lucide-react';
import { Modal } from '@/components/ui/Modal';
import { Button } from '@/components/ui/Button';
import type { Achievement } from '@/types';

interface AchievementModalProps {
  achievement: Achievement | null;
  onDismiss: () => void;
}

export default function AchievementModal({ achievement, onDismiss }: AchievementModalProps) {
  return (
    <Modal open={achievement !== null} onClose={onDismiss} className="max-w-sm text-center">
      {achievement && (
        <div className="flex flex-col items-center">
          {/* Glow pulse icon */}
          <motion.div
            className="relative mb-4"
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ delay: 0.1, type: 'spring', damping: 10 }}
          >
            <motion.div
              className="absolute inset-0 rounded-full bg-primary/20"
              animate={{ scale: [1, 1.4, 1], opacity: [0.6, 0, 0.6] }}
              transition={{ duration: 2, repeat: 0, ease: 'easeInOut' }}
            />
            <div className="w-16 h-16 rounded-full bg-primary/10 border-2 border-primary/30 flex items-center justify-center">
              <Trophy size={32} className="text-primary" aria-hidden="true" />
            </div>
          </motion.div>

          {/* Sparkle decoration */}
          <motion.div
            className="flex items-center gap-1 mb-2"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <Sparkles size={14} className="text-warning" aria-hidden="true" />
            <p className="text-xs font-bold tracking-[2px] uppercase text-primary">
              Achievement Unlocked!
            </p>
            <Sparkles size={14} className="text-warning" aria-hidden="true" />
          </motion.div>

          <h2 className="font-display text-2xl font-bold text-text-primary mb-2">
            {achievement.name}
          </h2>

          <p className="text-sm text-text-secondary mb-4">{achievement.description}</p>

          <div className="inline-block bg-primary/10 border border-primary/30 text-primary font-code text-sm font-bold px-4 py-1.5 rounded-full mb-6">
            +{achievement.xp_reward} XP
          </div>

          <Button onClick={onDismiss} variant="primary" size="lg" className="w-full">
            Awesome!
          </Button>
        </div>
      )}
    </Modal>
  );
}
