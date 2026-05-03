import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod/v4';
import { Link, useNavigate } from 'react-router-dom';
import { X, Bug } from 'lucide-react';
import { useAuthStore } from '@/stores/authStore';
import { Input, Button } from '@/components/ui';
import type { ApiError } from '@/types';

const registerSchema = z
  .object({
    username: z
      .string()
      .min(3, 'Username must be at least 3 characters')
      .max(32, 'Username must be at most 32 characters'),
    email: z.email('Please enter a valid email address'),
    password: z.string().min(8, 'Password must be at least 8 characters'),
    confirmPassword: z.string().min(1, 'Please confirm your password'),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword'],
  });

type RegisterFormData = z.infer<typeof registerSchema>;

export default function Register() {
  const navigate = useNavigate();
  const registerAction = useAuthStore((s) => s.register);
  const [apiError, setApiError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<RegisterFormData>({
    resolver: zodResolver(registerSchema),
    mode: 'onChange',
  });

  async function onSubmit(data: RegisterFormData) {
    setApiError(null);
    setIsSubmitting(true);
    try {
      await registerAction(data.username, data.email, data.password);
      navigate('/dashboard');
    } catch (error: unknown) {
      const apiErr = error as ApiError;
      setApiError(apiErr.message || 'Registration failed');
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center px-4 py-8 bg-surface-base bg-[radial-gradient(ellipse_at_center,_rgba(206,66,43,0.06)_0%,_transparent_70%)]">
      <div className="w-full max-w-[400px]">
        <div className="bg-surface-elevated border border-border rounded-xl p-6 sm:p-8">
          <div className="text-center mb-8">
            <Link
              to="/"
              className="inline-flex items-center gap-2 font-display text-2xl font-extrabold tracking-wide text-text-primary mb-2"
            >
              <Bug className="h-6 w-6 text-primary" aria-hidden="true" />
              <span>
                Rust<span className="text-primary">Gym</span>
              </span>
            </Link>
            <h1 className="font-display text-2xl font-bold text-text-primary mt-4">
              Create your account
            </h1>
            <p className="text-text-muted text-sm mt-1">Start your Rust learning journey today</p>
          </div>

          {apiError && (
            <div className="flex items-start gap-3 bg-error/10 border border-error/30 rounded-lg px-4 py-3 mb-6">
              <p className="text-error text-sm flex-1">{apiError}</p>
              <button
                type="button"
                onClick={() => setApiError(null)}
                className="text-error hover:text-error/80 transition-colors shrink-0 cursor-pointer"
                aria-label="Dismiss error"
              >
                <X className="h-4 w-4" aria-hidden="true" />
              </button>
            </div>
          )}

          <form onSubmit={handleSubmit(onSubmit)} className="space-y-5">
            <Input
              label="Username"
              type="text"
              autoComplete="username"
              placeholder="rustacean42"
              error={errors.username?.message}
              {...register('username')}
            />

            <Input
              label="Email"
              type="email"
              autoComplete="email"
              placeholder="you@example.com"
              error={errors.email?.message}
              {...register('email')}
            />

            <Input
              label="Password"
              type="password"
              autoComplete="new-password"
              placeholder="••••••••"
              error={errors.password?.message}
              {...register('password')}
            />

            <Input
              label="Confirm Password"
              type="password"
              autoComplete="new-password"
              placeholder="••••••••"
              error={errors.confirmPassword?.message}
              {...register('confirmPassword')}
            />

            <Button type="submit" className="w-full" size="lg" isLoading={isSubmitting}>
              {isSubmitting ? 'Creating account...' : 'Create Account'}
            </Button>
          </form>

          <p className="text-center text-text-muted text-sm mt-6">
            Already have an account?{' '}
            <Link
              to="/login"
              className="text-primary hover:text-primary-400 transition-colors font-medium"
            >
              Sign in
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
}
