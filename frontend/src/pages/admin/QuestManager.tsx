import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Plus, Trash2, ChevronDown, ChevronRight, Eye, EyeOff } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import type { AdminQuest, AdminLevel } from '@/types/admin';
import AdminLayout from './AdminLayout';

export default function QuestManager() {
  const queryClient = useQueryClient();
  const [expandedQuest, setExpandedQuest] = useState<string | null>(null);
  const [newQuestForm, setNewQuestForm] = useState(false);
  const [newLevelForm, setNewLevelForm] = useState<string | null>(null);

  const { data: quests, isLoading: questsLoading } = useQuery({
    queryKey: ['admin', 'quests'],
    queryFn: () => adminApi.listQuests(),
  });

  const { data: levels } = useQuery({
    queryKey: ['admin', 'levels'],
    queryFn: () => adminApi.listLevels(),
  });

  const createQuestMutation = useMutation({
    mutationFn: (data: Partial<AdminQuest>) => adminApi.createQuest(data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'quests'] });
      setNewQuestForm(false);
    },
  });

  const updateQuestMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<AdminQuest> }) =>
      adminApi.updateQuest(id, data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'quests'] });
    },
  });

  const deleteQuestMutation = useMutation({
    mutationFn: (id: string) => adminApi.deleteQuest(id),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'quests'] });
    },
  });

  const createLevelMutation = useMutation({
    mutationFn: (data: Partial<AdminLevel>) => adminApi.createLevel(data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'levels'] });
      setNewLevelForm(null);
    },
  });

  const updateLevelMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<AdminLevel> }) =>
      adminApi.updateLevel(id, data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'levels'] });
    },
  });

  const deleteLevelMutation = useMutation({
    mutationFn: (id: string) => adminApi.deleteLevel(id),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'levels'] });
    },
  });

  function getLevelsForQuest(questId: string): AdminLevel[] {
    return (levels ?? [])
      .filter((l) => l.quest_id === questId)
      .sort((a, b) => a.order_index - b.order_index);
  }

  return (
    <AdminLayout>
      <div>
        <div className="flex items-center justify-between mb-6">
          <h1 className="font-display text-2xl font-bold text-text-primary">Quest Manager</h1>
          <button
            onClick={() => setNewQuestForm(true)}
            className="flex items-center gap-2 bg-primary text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-primary-light transition-colors"
          >
            <Plus className="w-4 h-4" />
            New Quest
          </button>
        </div>

        {/* New Quest Form */}
        {newQuestForm && (
          <NewQuestForm
            onSubmit={(data) => createQuestMutation.mutate(data)}
            onCancel={() => setNewQuestForm(false)}
            orderIndex={(quests?.length ?? 0) + 1}
          />
        )}

        {/* Quest List */}
        {questsLoading ? (
          <div className="text-sm text-text-muted">Loading...</div>
        ) : (
          <div className="space-y-3">
            {quests?.map((quest) => {
              const isExpanded = expandedQuest === quest.id;
              const questLevels = getLevelsForQuest(quest.id);

              return (
                <div
                  key={quest.id}
                  className="bg-dark-card border border-border rounded-xl overflow-hidden"
                >
                  {/* Quest Header */}
                  <div className="flex items-center gap-3 p-4">
                    <button
                      onClick={() => setExpandedQuest(isExpanded ? null : quest.id)}
                      className="text-text-muted hover:text-text-primary transition-colors"
                      aria-label={isExpanded ? 'Collapse' : 'Expand'}
                    >
                      {isExpanded ? (
                        <ChevronDown className="w-4 h-4" />
                      ) : (
                        <ChevronRight className="w-4 h-4" />
                      )}
                    </button>
                    <span className="text-lg">{quest.icon || '📦'}</span>
                    <div className="flex-1 min-w-0">
                      <p className="text-sm font-medium text-text-primary">{quest.title}</p>
                      <p className="text-[11px] text-text-muted font-code">{quest.slug}</p>
                    </div>
                    <span className="text-xs text-text-muted">{questLevels.length} levels</span>
                    <button
                      onClick={() =>
                        updateQuestMutation.mutate({
                          id: quest.id,
                          data: { is_published: !quest.is_published },
                        })
                      }
                      className={`p-1.5 rounded transition-colors ${
                        quest.is_published
                          ? 'text-green hover:text-green/80'
                          : 'text-text-muted hover:text-text-primary'
                      }`}
                      title={quest.is_published ? 'Unpublish' : 'Publish'}
                      aria-label={quest.is_published ? 'Unpublish quest' : 'Publish quest'}
                    >
                      {quest.is_published ? (
                        <Eye className="w-4 h-4" />
                      ) : (
                        <EyeOff className="w-4 h-4" />
                      )}
                    </button>
                    <button
                      onClick={() => {
                        if (confirm('Delete this quest?')) {
                          deleteQuestMutation.mutate(quest.id);
                        }
                      }}
                      className="p-1.5 text-text-muted hover:text-red-400 transition-colors"
                      aria-label="Delete quest"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>

                  {/* Levels */}
                  {isExpanded && (
                    <div className="border-t border-border bg-dark-900/50 p-4 space-y-2">
                      {questLevels.map((level) => (
                        <div
                          key={level.id}
                          className="flex items-center gap-3 px-3 py-2 bg-dark-700/50 rounded-lg"
                        >
                          <span className="text-xs text-text-muted w-6 text-center">
                            {level.order_index}
                          </span>
                          <div className="flex-1 min-w-0">
                            <p className="text-sm text-text-primary">{level.title}</p>
                            <p className="text-[11px] text-text-muted font-code">{level.slug}</p>
                          </div>
                          <button
                            onClick={() =>
                              updateLevelMutation.mutate({
                                id: level.id,
                                data: { is_published: !level.is_published },
                              })
                            }
                            className={`p-1 rounded transition-colors ${
                              level.is_published
                                ? 'text-green hover:text-green/80'
                                : 'text-text-muted hover:text-text-primary'
                            }`}
                            aria-label={level.is_published ? 'Unpublish level' : 'Publish level'}
                          >
                            {level.is_published ? (
                              <Eye className="w-3.5 h-3.5" />
                            ) : (
                              <EyeOff className="w-3.5 h-3.5" />
                            )}
                          </button>
                          <button
                            onClick={() => {
                              if (confirm('Delete this level?')) {
                                deleteLevelMutation.mutate(level.id);
                              }
                            }}
                            className="p-1 text-text-muted hover:text-red-400 transition-colors"
                            aria-label="Delete level"
                          >
                            <Trash2 className="w-3.5 h-3.5" />
                          </button>
                        </div>
                      ))}

                      {/* Add Level */}
                      {newLevelForm === quest.id ? (
                        <NewLevelForm
                          questId={quest.id}
                          onSubmit={(data) => createLevelMutation.mutate(data)}
                          onCancel={() => setNewLevelForm(null)}
                          orderIndex={questLevels.length + 1}
                        />
                      ) : (
                        <button
                          onClick={() => setNewLevelForm(quest.id)}
                          className="flex items-center gap-2 px-3 py-2 text-xs text-text-muted hover:text-primary transition-colors"
                        >
                          <Plus className="w-3 h-3" />
                          Add Level
                        </button>
                      )}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </AdminLayout>
  );
}

function NewQuestForm({
  onSubmit,
  onCancel,
  orderIndex,
}: {
  onSubmit: (data: Partial<AdminQuest>) => void;
  onCancel: () => void;
  orderIndex: number;
}) {
  const [title, setTitle] = useState('');
  const [slug, setSlug] = useState('');

  return (
    <div className="bg-dark-card border border-border rounded-xl p-4 mb-4">
      <div className="flex items-center gap-3">
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Quest title"
          className="flex-1 bg-dark-700 border border-border rounded-lg px-3 py-2 text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-primary"
        />
        <input
          type="text"
          value={slug}
          onChange={(e) => setSlug(e.target.value)}
          placeholder="slug"
          className="w-48 bg-dark-700 border border-border rounded-lg px-3 py-2 text-sm text-text-primary font-code placeholder:text-text-muted focus:outline-none focus:border-primary"
        />
        <button
          onClick={() => onSubmit({ title, slug, order_index: orderIndex })}
          className="bg-primary text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-primary-light transition-colors"
        >
          Create
        </button>
        <button
          onClick={onCancel}
          className="text-sm text-text-muted hover:text-text-primary transition-colors"
        >
          Cancel
        </button>
      </div>
    </div>
  );
}

function NewLevelForm({
  questId,
  onSubmit,
  onCancel,
  orderIndex,
}: {
  questId: string;
  onSubmit: (data: Partial<AdminLevel>) => void;
  onCancel: () => void;
  orderIndex: number;
}) {
  const [title, setTitle] = useState('');
  const [slug, setSlug] = useState('');

  return (
    <div className="flex items-center gap-2 px-3 py-2 bg-dark-700/30 rounded-lg">
      <input
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="Level title"
        className="flex-1 bg-dark-700 border border-border rounded px-2 py-1.5 text-xs text-text-primary placeholder:text-text-muted focus:outline-none focus:border-primary"
      />
      <input
        type="text"
        value={slug}
        onChange={(e) => setSlug(e.target.value)}
        placeholder="slug"
        className="w-32 bg-dark-700 border border-border rounded px-2 py-1.5 text-xs text-text-primary font-code placeholder:text-text-muted focus:outline-none focus:border-primary"
      />
      <button
        onClick={() => onSubmit({ quest_id: questId, title, slug, order_index: orderIndex })}
        className="bg-primary text-white px-3 py-1.5 rounded text-xs font-medium hover:bg-primary-light transition-colors"
      >
        Add
      </button>
      <button
        onClick={onCancel}
        className="text-xs text-text-muted hover:text-text-primary transition-colors"
      >
        Cancel
      </button>
    </div>
  );
}
