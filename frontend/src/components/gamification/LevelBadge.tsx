interface LevelBadgeProps {
  level: number;
  size?: 'sm' | 'md' | 'lg';
}

const sizeClasses = {
  sm: 'w-6 h-6 text-[10px]',
  md: 'w-9 h-9 text-sm',
  lg: 'w-12 h-12 text-lg',
};

export default function LevelBadge({ level, size = 'md' }: LevelBadgeProps) {
  return (
    <div
      className={`${sizeClasses[size]} rounded-full bg-amber/20 border-2 border-amber/50 flex items-center justify-center font-bold text-amber`}
    >
      {level}
    </div>
  );
}
