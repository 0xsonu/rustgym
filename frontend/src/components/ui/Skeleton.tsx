import * as React from 'react';
import { cn } from '@/lib/utils';

interface SkeletonProps extends React.HTMLAttributes<HTMLDivElement> {
  className?: string;
}

const Skeleton = React.forwardRef<HTMLDivElement, SkeletonProps>(({ className, ...props }, ref) => {
  return (
    <div className={cn('bg-slate-800 rounded-md animate-pulse', className)} ref={ref} {...props} />
  );
});

Skeleton.displayName = 'Skeleton';

export { Skeleton };
export type { SkeletonProps };
