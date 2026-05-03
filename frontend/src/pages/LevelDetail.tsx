import { Link, useParams } from 'react-router-dom';
import { ArrowLeft, CheckCircle2, Zap } from 'lucide-react';
import { useLevelDetail } from '@/hooks';
import type { Task, Difficulty } from '@/types';
import { Card, Badge, Skeleton } from '@/components/ui';
import Navbar from '@/components/layout/Navbar';

const difficultyVariantMap: Record<
  Difficulty,
  'success' | 'info' | 'warning' | 'primary' | 'error'
> = {
  beginner: 'success',
  easy: 'info',
  medium: 'warning',
  hard: 'primary',
  advanced: 'error',
};

const difficultyLabelMap: Record<Difficulty, string> = {
  beginner: 'Beginner',
  easy: 'Easy',
  medium: 'Medium',
  hard: 'Hard',
  advanced: 'Advanced',
};

function TaskCard({ task }: { task: Task }) {
  const variant = difficultyVariantMap[task.difficulty];
  const label = difficultyLabelMap[task.difficulty];

  return (
    <Link to={`/tasks/${task.slug}`}>
      <Card interactive className="p-5 flex items-center gap-4">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-3 mb-1">
            <h3 className="font-display text-lg font-bold text-text-primary truncate">
              {task.title}
            </h3>
            {task.is_completed && <CheckCircle2 className="w-4.5 h-4.5 text-success shrink-0" />}
          </div>
          <div className="flex items-center gap-3">
            <Badge variant={variant}>{label}</Badge>
            <span className="inline-flex items-center gap-1 font-code text-xs text-warning font-bold">
              <Zap className="w-3 h-3" />+{task.xp_reward} XP
            </span>
          </div>
        </div>
      </Card>
    </Link>
  );
}

export default function LevelDetail() {
  const { questSlug, levelSlug } = useParams<{ questSlug: string; levelSlug: string }>();
  const { data: level, isLoading, error } = useLevelDetail(questSlug ?? '', levelSlug ?? '');

  const progress = level?.user_progress;
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-16 px-5 md:px-10">
          <div className="max-w-[900px] mx-auto">
            <Link
              to={`/quests/${questSlug ?? ''}`}
              className="inline-flex items-center gap-1.5 text-sm text-text-muted hover:text-text-primary transition-colors duration-200 mb-8 font-body"
            >
              <ArrowLeft className="w-4 h-4" />
              Back to {level?.quest_title ?? 'Quest'}
            </Link>

            {isLoading && (
              <div className="space-y-4">
                <Skeleton className="h-10 w-1/3 rounded-lg" />
                <Skeleton className="h-5 w-2/3 rounded-lg" />
                <Skeleton className="h-3 w-full max-w-md rounded-full" />
                <div className="space-y-3 mt-8">
                  {Array.from({ length: 5 }).map((_, i) => (
                    <Skeleton key={i} className="h-20 w-full rounded-xl" />
                  ))}
                </div>
              </div>
            )}

            {error && (
              <div className="text-center py-16">
                <p className="text-text-muted font-body">
                  Failed to load level. Please try again later.
                </p>
              </div>
            )}

            {level && (
              <>
                <div className="mb-10">
                  <h1 className="font-display text-3xl md:text-4xl font-extrabold text-text-primary mb-2">
                    {level.title}
                  </h1>
                  <p className="text-text-secondary text-lg font-body mb-4">{level.description}</p>
                  {progress && (
                    <div className="max-w-md">
                      <div className="flex items-center justify-between text-sm mb-2">
                        <span className="text-text-muted font-body">Progress</span>
                        <span className="text-text-secondary font-medium font-body tabular-nums">
                          {progress.tasks_completed}/{progress.tasks_total} tasks ·{' '}
                          {progressPercent}%
                        </span>
                      </div>
                      <div className="h-2.5 bg-surface-overlay rounded-full overflow-hidden">
                        <div
                          className="h-full rounded-full bg-gradient-to-r from-primary to-warning transition-all duration-500"
                          style={{ width: `${progressPercent}%` }}
                        />
                      </div>
                    </div>
                  )}
                </div>

                <div className="space-y-3">
                  {level.tasks.map((task) => (
                    <TaskCard key={task.id} task={task} />
                  ))}
                </div>
              </>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
