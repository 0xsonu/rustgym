import { Link, useParams } from 'react-router-dom';
import { Lock, Code2, ArrowLeft, CheckCircle, Circle, Loader2 } from 'lucide-react';
import { useQuestDetail } from '@/hooks';
import type { Level } from '@/types';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton } from '@/components/ui';

type LevelStatus = 'locked' | 'not-started' | 'in-progress' | 'completed';

function getLevelStatus(level: Level, index: number, previousCompleted: boolean): LevelStatus {
  if (index > 0 && !previousCompleted) return 'locked';
  if (level.user_progress?.is_completed) return 'completed';
  if (level.user_progress && level.user_progress.tasks_completed > 0) return 'in-progress';
  return 'not-started';
}

function LevelStatusIcon({ status }: { status: LevelStatus }) {
  switch (status) {
    case 'locked':
      return <Lock className="w-5 h-5 text-text-muted" aria-hidden="true" />;
    case 'completed':
      return <CheckCircle className="w-5 h-5 text-success" aria-hidden="true" />;
    case 'in-progress':
      return <Loader2 className="w-5 h-5 text-primary animate-spin" aria-hidden="true" />;
    case 'not-started':
      return <Circle className="w-5 h-5 text-text-muted" aria-hidden="true" />;
  }
}

const statusLabel: Record<LevelStatus, string> = {
  locked: 'Locked',
  'not-started': 'Not started',
  'in-progress': 'In progress',
  completed: 'Completed',
};

function LevelCard({
  level,
  questSlug,
  index,
  previousCompleted,
}: {
  level: Level;
  questSlug: string;
  index: number;
  previousCompleted: boolean;
}) {
  const status = getLevelStatus(level, index, previousCompleted);
  const progress = level.user_progress;
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  if (status === 'locked') {
    return (
      <Card className="opacity-50 cursor-not-allowed flex items-center gap-6">
        <div className="flex items-center justify-center w-12 h-12 rounded-xl bg-slate-800 shrink-0">
          <LevelStatusIcon status={status} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-0.5">
            <h3 className="font-display text-lg font-bold text-text-muted">{level.title}</h3>
            <Badge variant="default">{statusLabel[status]}</Badge>
          </div>
          <p className="text-sm text-text-muted line-clamp-1">{level.description}</p>
        </div>
        <div className="text-xs text-text-muted flex items-center gap-1 shrink-0">
          <Code2 className="w-3.5 h-3.5" aria-hidden="true" />
          {level.task_count} tasks
        </div>
      </Card>
    );
  }

  return (
    <Link to={`/quests/${questSlug}/levels/${level.slug}`} className="block">
      <Card interactive className="flex items-center gap-6">
        <div className="flex items-center justify-center w-12 h-12 rounded-xl bg-primary/10 border border-primary/30 shrink-0">
          <span className="text-lg font-bold text-primary">{index + 1}</span>
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-0.5">
            <h3 className="font-display text-lg font-bold text-text-primary">{level.title}</h3>
            <LevelStatusIcon status={status} />
            {status === 'completed' && <Badge variant="success">{statusLabel[status]}</Badge>}
            {status === 'in-progress' && <Badge variant="primary">{statusLabel[status]}</Badge>}
          </div>
          <p className="text-sm text-text-secondary line-clamp-1 mb-2">{level.description}</p>
          {progress && status === 'in-progress' && (
            <div className="flex items-center gap-3">
              <div className="h-1.5 bg-slate-800 rounded-full overflow-hidden flex-1 max-w-[200px]">
                <div
                  className="h-full rounded-full bg-gradient-to-r from-primary to-warning transition-all duration-500"
                  style={{ width: `${progressPercent}%` }}
                />
              </div>
              <span className="text-xs text-text-muted">
                {progress.tasks_completed}/{progress.tasks_total}
              </span>
            </div>
          )}
        </div>
        <div className="text-xs text-text-secondary flex items-center gap-1 shrink-0">
          <Code2 className="w-3.5 h-3.5" aria-hidden="true" />
          {level.task_count} tasks
        </div>
      </Card>
    </Link>
  );
}

function QuestDetailSkeleton() {
  return (
    <div>
      <Skeleton className="h-10 w-1/3 mb-4" />
      <Skeleton className="h-5 w-2/3 mb-8" />
      <div className="space-y-4">
        {Array.from({ length: 4 }).map((_, i) => (
          <Card key={i} className="flex items-center gap-6">
            <Skeleton className="w-12 h-12 rounded-xl shrink-0" />
            <div className="flex-1">
              <Skeleton className="h-5 w-1/3 mb-2" />
              <Skeleton className="h-4 w-2/3" />
            </div>
            <Skeleton className="h-4 w-16 shrink-0" />
          </Card>
        ))}
      </div>
    </div>
  );
}

export default function QuestDetail() {
  const { slug } = useParams<{ slug: string }>();
  const { data: quest, isLoading, error } = useQuestDetail(slug ?? '');

  const overallProgress = quest?.user_progress;
  const overallPercent = overallProgress
    ? Math.round((overallProgress.tasks_completed / overallProgress.tasks_total) * 100)
    : 0;

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-16 px-5 md:px-10">
          <div className="max-w-[900px] mx-auto">
            <Link
              to="/quests"
              className="inline-flex items-center gap-1.5 text-sm text-text-muted hover:text-text-primary transition-colors mb-8"
            >
              <ArrowLeft className="w-4 h-4" aria-hidden="true" />
              Back to Quests
            </Link>

            {isLoading && <QuestDetailSkeleton />}

            {error && (
              <div className="text-center py-16">
                <p className="text-text-muted">Failed to load quest. Please try again later.</p>
              </div>
            )}

            {quest && (
              <>
                <Card className="mb-10">
                  <div className="flex items-start gap-4 mb-4">
                    <div className="flex items-center justify-center w-14 h-14 rounded-xl bg-primary/10 border border-primary/30 shrink-0">
                      <Code2 className="w-7 h-7 text-primary" aria-hidden="true" />
                    </div>
                    <div className="flex-1">
                      <h1 className="font-display text-3xl md:text-4xl font-extrabold text-text-primary mb-2">
                        {quest.title}
                      </h1>
                      <p className="text-text-secondary text-lg font-body">{quest.description}</p>
                    </div>
                  </div>
                  {overallProgress && (
                    <div className="max-w-md">
                      <div className="flex items-center justify-between text-sm mb-2">
                        <span className="text-text-muted">Overall Progress</span>
                        <span className="text-text-secondary font-medium">
                          {overallProgress.tasks_completed}/{overallProgress.tasks_total} tasks ·{' '}
                          {overallPercent}%
                        </span>
                      </div>
                      <div className="h-2.5 bg-slate-800 rounded-full overflow-hidden">
                        <div
                          className="h-full rounded-full bg-gradient-to-r from-primary to-warning transition-all duration-500"
                          style={{ width: `${overallPercent}%` }}
                        />
                      </div>
                    </div>
                  )}
                </Card>

                <h2 className="font-display text-xl font-bold text-text-primary mb-4">Levels</h2>
                <div className="space-y-4">
                  {quest.levels.map((level, index) => {
                    const prevLevel = quest.levels[index - 1];
                    const previousCompleted = prevLevel?.user_progress?.is_completed ?? true;
                    return (
                      <LevelCard
                        key={level.id}
                        level={level}
                        questSlug={quest.slug}
                        index={index}
                        previousCompleted={previousCompleted}
                      />
                    );
                  })}
                </div>
              </>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
