/* eslint-disable react-refresh/only-export-components */
import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const cardVariants = cva(
  'rounded-xl border border-border bg-surface-elevated p-6 transition-all duration-200',
  {
    variants: {
      interactive: {
        true: 'cursor-pointer hover:shadow-md hover:border-border-light',
        false: '',
      },
    },
    defaultVariants: { interactive: false },
  },
);

interface CardProps
  extends React.HTMLAttributes<HTMLDivElement>, VariantProps<typeof cardVariants> {}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  ({ className, interactive, ...props }, ref) => {
    return <div className={cn(cardVariants({ interactive, className }))} ref={ref} {...props} />;
  },
);

Card.displayName = 'Card';

export { Card, cardVariants };
export type { CardProps };
