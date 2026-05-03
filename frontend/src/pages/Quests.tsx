import { Link } from 'react-router-dom';
import { Lock, Layers, Code2, CheckCircle } from 'lucide-react';
import { useQuests } from '@/hooks';
import type { Quest } from '@/types';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton } from '@/components/ui';

type QuestDifficulty = 'beginner' | 'intermediate' | 'advanced';

function getQuestDifficulty(quest: Quest): QuestDifficulty {
  if (quest.order_index <= 2) return 'beginner';
  if (quest.order_index <= 5) return 'intermediate';
  return 'advanced';
}

const difficultyBadgeVariant: Record<QuestDifficulty, 'success' | 'warning' | 'error'> = {
  beginner: 'success',
  intermediate: 'warning',
  advanced: 'error',
};

type QuestStatus = 'locked' | 'available' | 'in-progress' | 'completed';

function getQuestStatus(quest: Quest): QuestStatus {
  if (!quest.user_progress && quest.prerequisite_quest_id !== null) return 'locked';
  if (quest.user_progress?.is_completed) return 'completed';
  if (quest.user_progress && quest.user_progress.tasks_completed > 0) return 'in-progress';
  return 'available';
}

const statusBorderClasses: Record<QuestStatus, string> = {
  locked: 'border-border opacity-50',
  available: 'border-border',
  'in-progress': 'border-primary',
  completed: 'border-success',
};

function QuestCard({ quest }: { quest: Quest }) {
  const progress = quest.user_progress;
  const status = getQuestStatus(quest);
  const difficulty = getQuestDifficulty(quest);
  const progressPercent = progress
    ? Math.round((progress.tasks_completed / progress.tasks_total) * 100)
    : 0;

  if (status === 'locked') {
    return (
      <Card className={`${statusBorderClasses[status]} cursor-not-allowed`}>
        <div className="flex items-center justify-between mb-4">
          <Lock className="w-6 h-6 text-text-muted" aria-hidden="true" />
          <Badge variant={difficultyBadgeVariant[difficulty]}>{difficulty}</Badge>
        </div>
        <h3 className="font-display text-xl font-bold text-text-muted mb-2">{quest.title}</h3>
        <p className="text-sm text-text-muted line-clamp-2 mb-4">{quest.description}</p>
        <div className="flex items-center gap-4 text-xs text-text-muted">
          <span className="flex items-center gap-1">
            <Layers className="w-3.5 h-3.5" aria-hidden="true" />
            {quest.level_count} levels
          </span>
          <span className="flex items-center gap-1">
            <Code2 className="w-3.5 h-3.5" aria-hidden="true" />
            {quest.task_count} tasks
          </span>
        </div>
      </Card>
    );
  }

  return (
    <Link to={`/quests/${quest.slug}`} className="block">
      <Card
        interactive
        className={`${statusBorderClasses[status]} hover:-translate-y-1 transition-all duration-200`}
      >
        <div className="flex items-center justify-between mb-4">
          {status === 'completed' ? (
            <CheckCircle className="w-6 h-6 text-success" aria-hidden="true" />
          ) : (
            <Layers className="w-6 h-6 text-primary" aria-hidden="true" />
          )}
          <Badge variant={difficultyBadgeVariant[difficulty]}>{difficulty}</Badge>
        </div>
        <h3 className="font-display text-xl font-bold text-text-primary mb-2">{quest.title}</h3>
        <p className="text-sm text-text-secondary line-clamp-2 mb-4">{quest.description}</p>
        <div className="flex items-center gap-4 text-xs text-text-secondary mb-4">
          <span className="flex items-center gap-1">
            <Layers className="w-3.5 h-3.5" aria-hidden="true" />
            {quest.level_count} levels
          </span>
          <span className="flex items-center gap-1">
            <Code2 className="w-3.5 h-3.5" aria-hidden="true" />
            {quest.task_count} tasks
          </span>
        </div>
        {progress && status === 'in-progress' && (
          <div>
            <div className="flex items-center justify-between text-xs mb-1.5">
              <span className="text-text-muted">
                {progress.tasks_completed}/{progress.tasks_total} tasks
              </span>
              <span className="text-text-secondary font-medium">{progressPercent}%</span>
            </div>
            <div className="h-2 bg-slate-800 rounded-full overflow-hidden">
              <div
                className="h-full rounded-full bg-gradient-to-r from-primary to-warning transition-all duration-500"
                style={{ width: `${progressPercent}%` }}
              />
            </div>
          </div>
        )}
        {status === 'completed' && (
          <div className="flex items-center gap-1.5 text-xs text-success font-medium">
            <CheckCircle className="w-3.5 h-3.5" aria-hidden="true" />
            Completed
          </div>
        )}
      </Card>
    </Link>
  );
}

function QuestsSkeleton() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {Array.from({ length: 6 }).map((_, i) => (
        <Card key={i}>
          <div className="flex items-center justify-between mb-4">
            <Skeleton className="w-6 h-6 rounded-full" />
            <Skeleton className="w-20 h-5" />
          </div>
          <Skeleton className="h-6 w-3/4 mb-2" />
          <Skeleton className="h-4 w-full mb-1" />
          <Skeleton className="h-4 w-2/3 mb-4" />
          <div className="flex items-center gap-4">
            <Skeleton className="h-3 w-16" />
            <Skeleton className="h-3 w-16" />
          </div>
        </Card>
      ))}
    </div>
  );
}

export default function Quests() {
  const { data: quests, isLoading, error } = useQuests();

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-16 px-5 md:px-10">
          <div className="max-w-[1280px] mx-auto">
            <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-3">
              CURRICULUM
            </p>
            <h1 className="font-display text-4xl md:text-5xl font-extrabold text-text-primary mb-3">
              Quest <span className="text-primary">Map</span>
            </h1>
            <p className="text-text-secondary text-lg mb-10 max-w-2xl font-body">
              Master Rust step by step. Complete quests to unlock new topics and earn XP along the
              way.
            </p>

            {isLoading && <QuestsSkeleton />}

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
