/* eslint-disable react-refresh/only-export-components */
import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const badgeVariants = cva(
  'inline-flex items-center gap-1 font-semibold rounded-full border px-2.5 py-0.5 text-xs',
  {
    variants: {
      variant: {
        default: 'bg-slate-800 text-text-secondary border-border',
        success: 'bg-success/10 text-success border-success/30',
        warning: 'bg-warning/10 text-warning border-warning/30',
        error: 'bg-error/10 text-error border-error/30',
        info: 'bg-info/10 text-info border-info/30',
        primary: 'bg-primary/10 text-primary border-primary/30',
      },
    },
    defaultVariants: { variant: 'default' },
  },
);

interface BadgeProps
  extends React.HTMLAttributes<HTMLSpanElement>, VariantProps<typeof badgeVariants> {}

const Badge = React.forwardRef<HTMLSpanElement, BadgeProps>(
  ({ className, variant, children, ...props }, ref) => {
    return (
      <span className={cn(badgeVariants({ variant, className }))} ref={ref} {...props}>
        {children}
      </span>
    );
  },
);

Badge.displayName = 'Badge';

export { Badge, badgeVariants };
export type { BadgeProps };
