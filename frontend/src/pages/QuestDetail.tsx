import { Link, useParams } from 'react-router-dom';
import { Lock, Code2, ArrowLeft } from 'lucide-react';
import { useQuestDetail } from '@/hooks';
import type { Level } from '@/types';
import Navbar from '@/components/layout/Navbar';

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
  const progress = level.user_progress;
  const isLocked = index > 0 && !previousCompleted;
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  if (isLocked) {
    return (
      <div className="bg-dark-card border border-border rounded-2xl p-6 opacity-60 cursor-not-allowed flex items-center gap-6">
        <div className="flex items-center justify-center w-12 h-12 rounded-xl bg-dark-700 shrink-0">
          <Lock className="w-5 h-5 text-text-muted" />
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="font-display text-lg font-bold text-text-muted">{level.title}</h3>
          <p className="text-sm text-text-muted line-clamp-1">{level.description}</p>
        </div>
        <div className="text-xs text-text-muted flex items-center gap-1 shrink-0">
          <Code2 className="w-3.5 h-3.5" />
          {level.task_count} tasks
        </div>
      </div>
    );
  }

  return (
    <Link
      to={`/quests/${questSlug}/levels/${level.slug}`}
      className="bg-dark-card border border-border rounded-2xl p-6 hover:border-border-light hover:-translate-y-[3px] transition-all flex items-center gap-6"
    >
      <div className="flex items-center justify-center w-12 h-12 rounded-xl bg-primary/10 border border-primary/30 shrink-0">
        <span className="text-lg font-bold text-primary">{index + 1}</span>
      </div>
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <h3 className="font-display text-lg font-bold text-text-primary">{level.title}</h3>
          {progress?.is_completed && (
            <span className="text-[10px] font-bold text-green bg-green/10 border border-green/30 px-2 py-0.5 rounded-full">
              Done
            </span>
          )}
        </div>
        <p className="text-sm text-text-secondary line-clamp-1 mb-2">{level.description}</p>
        {progress && (
          <div className="flex items-center gap-3">
            <div className="h-1.5 bg-dark-700 rounded-full overflow-hidden flex-1 max-w-[200px]">
              <div
                className="h-full rounded-full bg-gradient-to-r from-primary to-amber transition-all duration-500"
                style={{ width: `${progressPercent}%` }}
              />
            </div>
            <span className="text-xs text-text-muted">{progressPercent}%</span>
          </div>
        )}
      </div>
      <div className="text-xs text-text-secondary flex items-center gap-1 shrink-0">
        <Code2 className="w-3.5 h-3.5" />
        {level.task_count} tasks
      </div>
    </Link>
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
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-16 px-5 md:px-10">
          <div className="max-w-[900px] mx-auto">
            <Link
              to="/quests"
              className="inline-flex items-center gap-1.5 text-sm text-text-muted hover:text-text-primary transition-colors mb-8"
            >
              <ArrowLeft className="w-4 h-4" />
              Back to Quests
            </Link>

            {isLoading && (
              <div className="animate-pulse">
                <div className="h-10 bg-dark-card rounded w-1/3 mb-4" />
                <div className="h-5 bg-dark-card rounded w-2/3 mb-8" />
                <div className="space-y-4">
                  {Array.from({ length: 4 }).map((_, i) => (
                    <div key={i} className="h-24 bg-dark-card border border-border rounded-2xl" />
                  ))}
                </div>
              </div>
            )}

            {error && (
              <div className="text-center py-16">
                <p className="text-text-muted">Failed to load quest. Please try again later.</p>
              </div>
            )}

            {quest && (
              <>
                <div className="mb-10">
                  <div className="flex items-center gap-4 mb-3">
                    <span className="text-4xl">{quest.icon || '📦'}</span>
                    <div>
                      <h1 className="font-display text-3xl md:text-4xl font-extrabold text-text-primary">
                        {quest.title}
                      </h1>
                    </div>
                  </div>
                  <p className="text-text-secondary text-lg mb-4">{quest.description}</p>
                  {overallProgress && (
                    <div className="max-w-md">
                      <div className="flex items-center justify-between text-sm mb-2">
                        <span className="text-text-muted">Overall Progress</span>
                        <span className="text-text-secondary font-medium">
                          {overallProgress.tasks_completed}/{overallProgress.tasks_total} tasks ·{' '}
                          {overallPercent}%
                        </span>
                      </div>
                      <div className="h-2.5 bg-dark-700 rounded-full overflow-hidden">
                        <div
                          className="h-full rounded-full bg-gradient-to-r from-primary to-amber transition-all duration-500"
                          style={{ width: `${overallPercent}%` }}
                        />
                      </div>
                    </div>
                  )}
                </div>

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
