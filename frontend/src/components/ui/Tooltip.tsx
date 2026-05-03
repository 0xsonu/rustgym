import * as React from 'react';
import { cn } from '@/lib/utils';

interface TooltipProps {
  content: string;
  side?: 'top' | 'bottom' | 'left' | 'right';
  children: React.ReactNode;
}

const positionStyles: Record<NonNullable<TooltipProps['side']>, string> = {
  top: 'bottom-full left-1/2 -translate-x-1/2 mb-2 group-hover:-translate-y-0.5',
  bottom: 'top-full left-1/2 -translate-x-1/2 mt-2 group-hover:translate-y-0.5',
  left: 'right-full top-1/2 -translate-y-1/2 mr-2',
  right: 'left-full top-1/2 -translate-y-1/2 ml-2',
};

const Tooltip = React.forwardRef<
  HTMLDivElement,
  TooltipProps & React.HTMLAttributes<HTMLDivElement>
>(({ content, side = 'top', children, className, ...props }, ref) => {
  return (
    <div ref={ref} className={cn('relative inline-flex group', className)} {...props}>
      {children}
      <span
        role="tooltip"
        className={cn(
          'absolute z-50 whitespace-nowrap pointer-events-none',
          'bg-slate-800 text-text-primary text-xs px-2 py-1 rounded-md shadow-md',
          'opacity-0 group-hover:opacity-100 transition-all duration-150',
          positionStyles[side],
        )}
      >
        {content}
      </span>
    </div>
  );
});

Tooltip.displayName = 'Tooltip';

export { Tooltip };
export type { TooltipProps };
