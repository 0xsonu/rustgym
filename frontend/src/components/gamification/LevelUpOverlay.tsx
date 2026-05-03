import { useEffect } from 'react';
import { motion } from 'framer-motion';
import { ArrowUp } from 'lucide-react';
import { Modal } from '@/components/ui/Modal';

interface LevelUpOverlayProps {
  newLevel: number | null;
  onDismiss: () => void;
}

export default function LevelUpOverlay({ newLevel, onDismiss }: LevelUpOverlayProps) {
  // Auto-dismiss after 3 seconds
  useEffect(() => {
    if (newLevel !== null) {
      const timer = setTimeout(onDismiss, 3000);
      return () => clearTimeout(timer);
    }
  }, [newLevel, onDismiss]);

  return (
    <Modal open={newLevel !== null} onClose={onDismiss} className="max-w-xs text-center">
      {newLevel !== null && (
        <div className="flex flex-col items-center">
          {/* Glow pulse animation (max 2 seconds) */}
          <motion.div
            className="relative mb-4"
            initial={{ scale: 0.3, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ type: 'spring', damping: 12, stiffness: 150 }}
          >
            <motion.div
              className="absolute inset-0 rounded-full bg-warning/20"
              animate={{ scale: [1, 1.5, 1], opacity: [0.5, 0, 0.5] }}
              transition={{ duration: 2, repeat: 0, ease: 'easeInOut' }}
            />
            <div className="w-24 h-24 rounded-full bg-warning/20 border-4 border-warning flex items-center justify-center">
              <span className="font-display text-4xl font-extrabold text-warning">{newLevel}</span>
            </div>
          </motion.div>

          <motion.div
            className="flex items-center gap-1.5 mb-2"
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.2 }}
          >
            <ArrowUp size={16} className="text-warning" aria-hidden="true" />
            <p className="text-xs font-bold tracking-[3px] uppercase text-warning">Level Up!</p>
            <ArrowUp size={16} className="text-warning" aria-hidden="true" />
          </motion.div>

          <motion.p
            className="text-text-secondary text-sm"
            initial={{ y: 10, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.4 }}
          >
            Keep going, Rustacean!
          </motion.p>
        </div>
      )}
    </Modal>
  );
}
