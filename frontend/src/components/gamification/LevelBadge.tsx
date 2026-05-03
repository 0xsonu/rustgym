import { Star } from 'lucide-react';

interface LevelBadgeProps {
  level: number;
  size?: 'sm' | 'md' | 'lg';
}

const sizeClasses = {
  sm: 'w-6 h-6 text-[10px]',
  md: 'w-9 h-9 text-sm',
  lg: 'w-12 h-12 text-lg',
};

const iconSizes = {
  sm: 8,
  md: 10,
  lg: 14,
};

export default function LevelBadge({ level, size = 'md' }: LevelBadgeProps) {
  return (
    <div
      className={`${sizeClasses[size]} relative rounded-full bg-warning/20 border-2 border-warning/50 flex items-center justify-center font-bold text-warning`}
    >
      <Star
        size={iconSizes[size]}
        className="absolute -top-1 -right-1 text-warning fill-warning"
        aria-hidden="true"
      />
      <span>{level}</span>
    </div>
  );
}
