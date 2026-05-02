import { Link } from 'react-router-dom';
import { Lock, Layers, Code2 } from 'lucide-react';
import { useQuests } from '@/hooks';
import type { Quest } from '@/types';
import Navbar from '@/components/layout/Navbar';

function QuestCard({ quest }: { quest: Quest }) {
  const progress = quest.user_progress;
  const isLocked = !progress && quest.prerequisite_quest_id !== null;
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  if (isLocked) {
    return (
      <div className="bg-dark-card border border-border rounded-2xl p-6 opacity-60 cursor-not-allowed">
        <div className="flex items-center justify-between mb-4">
          <span className="text-3xl grayscale">{quest.icon || '📦'}</span>
          <Lock className="w-5 h-5 text-text-muted" />
        </div>
        <h3 className="font-display text-xl font-bold text-text-muted mb-2">{quest.title}</h3>
        <p className="text-sm text-text-muted line-clamp-2 mb-4">{quest.description}</p>
        <div className="flex items-center gap-4 text-xs text-text-muted">
          <span className="flex items-center gap-1">
            <Layers className="w-3.5 h-3.5" />
            {quest.level_count} levels
          </span>
          <span className="flex items-center gap-1">
            <Code2 className="w-3.5 h-3.5" />
            {quest.task_count} tasks
          </span>
        </div>
      </div>
    );
  }

  return (
    <Link
      to={`/quests/${quest.slug}`}
      className="bg-dark-card border border-border rounded-2xl p-6 hover:border-border-light hover:-translate-y-[3px] transition-all block"
    >
      <div className="flex items-center justify-between mb-4">
        <span className="text-3xl">{quest.icon || '📦'}</span>
        {progress?.is_completed && (
          <span className="text-xs font-bold text-green bg-green/10 border border-green/30 px-2 py-0.5 rounded-full">
            Complete
          </span>
        )}
      </div>
      <h3 className="font-display text-xl font-bold text-text-primary mb-2">{quest.title}</h3>
      <p className="text-sm text-text-secondary line-clamp-2 mb-4">{quest.description}</p>
      <div className="flex items-center gap-4 text-xs text-text-secondary mb-4">
        <span className="flex items-center gap-1">
          <Layers className="w-3.5 h-3.5" />
          {quest.level_count} levels
        </span>
        <span className="flex items-center gap-1">
          <Code2 className="w-3.5 h-3.5" />
          {quest.task_count} tasks
        </span>
      </div>
      {progress && (
        <div>
          <div className="flex items-center justify-between text-xs mb-1.5">
            <span className="text-text-muted">Progress</span>
            <span className="text-text-secondary font-medium">{progressPercent}%</span>
          </div>
          <div className="h-2 bg-dark-700 rounded-full overflow-hidden">
            <div
              className="h-full rounded-full bg-gradient-to-r from-primary to-amber transition-all duration-500"
              style={{ width: `${progressPercent}%` }}
            />
          </div>
        </div>
      )}
    </Link>
  );
}

export default function Quests() {
  const { data: quests, isLoading, error } = useQuests();

  return (
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-16 px-5 md:px-10">
          <div className="max-w-[1200px] mx-auto">
            <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-3">
              CURRICULUM
            </p>
            <h1 className="font-display text-4xl md:text-5xl font-extrabold text-text-primary mb-3">
              Quest <span className="text-primary">Map</span>
            </h1>
            <p className="text-text-secondary text-lg mb-10 max-w-2xl">
              Master Rust step by step. Complete quests to unlock new topics and earn XP along the
              way.
            </p>

            {isLoading && (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {Array.from({ length: 6 }).map((_, i) => (
                  <div
                    key={i}
                    className="bg-dark-card border border-border rounded-2xl p-6 animate-pulse h-[220px]"
                  />
                ))}
              </div>
            )}

            {error && (
              <div className="text-center py-16">
                <p className="text-text-muted">Failed to load quests. Please try again later.</p>
              </div>
            )}

            {quests && (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {quests.map((quest) => (
                  <QuestCard key={quest.id} quest={quest} />
                ))}
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
