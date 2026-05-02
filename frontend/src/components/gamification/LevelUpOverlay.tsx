import { useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

interface LevelUpOverlayProps {
  newLevel: number | null;
  onDismiss: () => void;
}

export default function LevelUpOverlay({ newLevel, onDismiss }: LevelUpOverlayProps) {
  useEffect(() => {
    if (newLevel !== null) {
      const timer = setTimeout(onDismiss, 3000);
      return () => clearTimeout(timer);
    }
  }, [newLevel, onDismiss]);

  return (
    <AnimatePresence>
      {newLevel !== null && (
        <motion.div
          className="fixed inset-0 z-[1000] flex items-center justify-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.3 }}
          onClick={onDismiss}
        >
          <div className="absolute inset-0 bg-dark-950/80 backdrop-blur-sm" />

          <motion.div
            className="relative text-center"
            initial={{ scale: 0.3, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 1.5, opacity: 0 }}
            transition={{ type: 'spring', damping: 12, stiffness: 150 }}
          >
            <motion.p
              className="text-xs font-bold tracking-[3px] uppercase text-amber mb-4"
              initial={{ y: 20, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              transition={{ delay: 0.2 }}
            >
              Level Up!
            </motion.p>

            <motion.div
              className="w-24 h-24 rounded-full bg-amber/20 border-4 border-amber flex items-center justify-center mx-auto mb-4"
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={{ delay: 0.3, type: 'spring', damping: 10 }}
            >
              <span className="font-display text-4xl font-extrabold text-amber">{newLevel}</span>
            </motion.div>

            <motion.p
              className="text-text-secondary text-sm"
              initial={{ y: 10, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              transition={{ delay: 0.5 }}
            >
              Keep going, Rustacean!
            </motion.p>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
