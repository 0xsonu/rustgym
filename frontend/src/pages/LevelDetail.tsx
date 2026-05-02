import { Link, useParams } from 'react-router-dom';
import { ArrowLeft, CheckCircle2 } from 'lucide-react';
import { useLevelDetail } from '@/hooks';
import type { Task, Difficulty } from '@/types';
import Navbar from '@/components/layout/Navbar';

const difficultyConfig: Record<Difficulty, { label: string; className: string }> = {
  beginner: { label: 'Beginner', className: 'bg-green/10 text-green border-green/30' },
  easy: { label: 'Easy', className: 'bg-blue/10 text-blue border-blue/30' },
  medium: { label: 'Medium', className: 'bg-amber/10 text-amber border-amber/30' },
  hard: { label: 'Hard', className: 'bg-primary/10 text-primary border-primary/30' },
  advanced: {
    label: 'Advanced',
    className: 'bg-purple-400/10 text-purple-400 border-purple-400/30',
  },
};

function TaskCard({ task }: { task: Task }) {
  const config = difficultyConfig[task.difficulty];

  return (
    <Link
      to={`/tasks/${task.slug}`}
      className="bg-dark-card border border-border rounded-2xl p-5 hover:border-border-light hover:-translate-y-[3px] transition-all flex items-center gap-4"
    >
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-3 mb-1">
          <h3 className="font-display text-lg font-bold text-text-primary truncate">
            {task.title}
          </h3>
          {task.is_completed && <CheckCircle2 className="w-4.5 h-4.5 text-green shrink-0" />}
        </div>
        <div className="flex items-center gap-3">
          <span
            className={`text-[11px] font-bold px-2.5 py-0.5 rounded-full border ${config.className}`}
          >
            {config.label}
          </span>
          <span className="font-code text-xs text-amber font-bold">+{task.xp_reward} XP</span>
        </div>
      </div>
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
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-16 px-5 md:px-10">
          <div className="max-w-[900px] mx-auto">
            <Link
              to={`/quests/${questSlug ?? ''}`}
              className="inline-flex items-center gap-1.5 text-sm text-text-muted hover:text-text-primary transition-colors mb-8"
            >
              <ArrowLeft className="w-4 h-4" />
              Back to {level?.quest_title ?? 'Quest'}
            </Link>

            {isLoading && (
              <div className="animate-pulse">
                <div className="h-10 bg-dark-card rounded w-1/3 mb-4" />
                <div className="h-5 bg-dark-card rounded w-2/3 mb-8" />
                <div className="space-y-3">
                  {Array.from({ length: 5 }).map((_, i) => (
                    <div key={i} className="h-20 bg-dark-card border border-border rounded-2xl" />
                  ))}
                </div>
              </div>
            )}

            {error && (
              <div className="text-center py-16">
                <p className="text-text-muted">Failed to load level. Please try again later.</p>
              </div>
            )}

            {level && (
              <>
                <div className="mb-10">
                  <h1 className="font-display text-3xl md:text-4xl font-extrabold text-text-primary mb-2">
                    {level.title}
                  </h1>
                  <p className="text-text-secondary text-lg mb-4">{level.description}</p>
                  {progress && (
                    <div className="max-w-md">
                      <div className="flex items-center justify-between text-sm mb-2">
                        <span className="text-text-muted">Progress</span>
                        <span className="text-text-secondary font-medium">
                          {progress.tasks_completed}/{progress.tasks_total} tasks ·{' '}
                          {progressPercent}%
                        </span>
                      </div>
                      <div className="h-2.5 bg-dark-700 rounded-full overflow-hidden">
                        <div
                          className="h-full rounded-full bg-gradient-to-r from-primary to-amber transition-all duration-500"
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
