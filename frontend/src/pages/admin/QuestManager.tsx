import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Plus, Trash2, ChevronDown, ChevronRight, Eye, EyeOff, Compass } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import type { AdminQuest, AdminLevel } from '@/types/admin';
import { Button, Input, Card, Modal } from '@/components/ui';
import AdminLayout from './AdminLayout';

export default function QuestManager() {
  const queryClient = useQueryClient();
  const [expandedQuest, setExpandedQuest] = useState<string | null>(null);
  const [newQuestForm, setNewQuestForm] = useState(false);
  const [newLevelForm, setNewLevelForm] = useState<string | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<{
    type: 'quest' | 'level';
    id: string;
    name: string;
  } | null>(null);

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
      setDeleteTarget(null);
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
      setDeleteTarget(null);
    },
  });

  function getLevelsForQuest(questId: string): AdminLevel[] {
    return (levels ?? [])
      .filter((l) => l.quest_id === questId)
      .sort((a, b) => a.order_index - b.order_index);
  }

  function handleConfirmDelete() {
    if (!deleteTarget) return;
    if (deleteTarget.type === 'quest') {
      deleteQuestMutation.mutate(deleteTarget.id);
    } else {
      deleteLevelMutation.mutate(deleteTarget.id);
    }
  }

  return (
    <AdminLayout>
      <div>
        <div className="flex items-center justify-between mb-6">
          <h1 className="font-display text-2xl font-bold text-text-primary">Quest Manager</h1>
          <Button onClick={() => setNewQuestForm(true)}>
            <Plus className="w-4 h-4" />
            New Quest
          </Button>
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
          <div className="text-sm text-text-muted font-body">Loading...</div>
        ) : (
          <div className="space-y-3">
            {quests?.map((quest) => {
              const isExpanded = expandedQuest === quest.id;
              const questLevels = getLevelsForQuest(quest.id);

              return (
                <Card key={quest.id} className="p-0 overflow-hidden">
                  {/* Quest Header */}
                  <div className="flex items-center gap-3 p-4">
                    <button
                      onClick={() => setExpandedQuest(isExpanded ? null : quest.id)}
                      className="text-text-muted hover:text-text-primary transition-colors duration-200 cursor-pointer"
                      aria-label={isExpanded ? 'Collapse' : 'Expand'}
                    >
                      {isExpanded ? (
                        <ChevronDown className="w-4 h-4" />
                      ) : (
                        <ChevronRight className="w-4 h-4" />
                      )}
                    </button>
                    <div className="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center">
                      <Compass className="w-4 h-4 text-primary" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="text-sm font-medium text-text-primary font-body">
                        {quest.title}
                      </p>
                      <p className="text-[11px] text-text-muted font-code">{quest.slug}</p>
                    </div>
                    <span className="text-xs text-text-muted font-body">
                      {questLevels.length} levels
                    </span>
                    <button
                      onClick={() =>
                        updateQuestMutation.mutate({
                          id: quest.id,
                          data: { is_published: !quest.is_published },
                        })
                      }
                      className={`p-1.5 rounded transition-colors duration-200 cursor-pointer ${
                        quest.is_published
                          ? 'text-success hover:text-success/80'
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
                      onClick={() =>
                        setDeleteTarget({ type: 'quest', id: quest.id, name: quest.title })
                      }
                      className="p-1.5 text-text-muted hover:text-error transition-colors duration-200 cursor-pointer"
                      aria-label="Delete quest"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>

                  {/* Levels */}
                  {isExpanded && (
                    <div className="border-t border-border bg-surface-base/50 p-4 space-y-2">
                      {questLevels.map((level) => (
                        <div
                          key={level.id}
                          className="flex items-center gap-3 px-3 py-2 bg-surface-overlay/50 rounded-lg"
                        >
                          <span className="text-xs text-text-muted w-6 text-center font-code">
                            {level.order_index}
                          </span>
                          <div className="flex-1 min-w-0">
                            <p className="text-sm text-text-primary font-body">{level.title}</p>
                            <p className="text-[11px] text-text-muted font-code">{level.slug}</p>
                          </div>
                          <button
                            onClick={() =>
                              updateLevelMutation.mutate({
                                id: level.id,
                                data: { is_published: !level.is_published },
                              })
                            }
                            className={`p-1 rounded transition-colors duration-200 cursor-pointer ${
                              level.is_published
                                ? 'text-success hover:text-success/80'
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
                            onClick={() =>
                              setDeleteTarget({ type: 'level', id: level.id, name: level.title })
                            }
                            className="p-1 text-text-muted hover:text-error transition-colors duration-200 cursor-pointer"
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
                          className="flex items-center gap-2 px-3 py-2 text-xs text-text-muted hover:text-primary transition-colors duration-200 cursor-pointer"
                        >
                          <Plus className="w-3 h-3" />
                          Add Level
                        </button>
                      )}
                    </div>
                  )}
                </Card>
              );
            })}
          </div>
        )}

        {/* Delete Confirmation Modal */}
        <Modal
          open={deleteTarget !== null}
          onClose={() => setDeleteTarget(null)}
          title={`Delete ${deleteTarget?.type === 'quest' ? 'Quest' : 'Level'}`}
        >
          <p className="text-sm text-text-secondary font-body mb-6">
            Are you sure you want to delete{' '}
            <span className="font-semibold text-text-primary">{deleteTarget?.name}</span>? This
            action cannot be undone.
          </p>
          <div className="flex justify-end gap-3">
            <Button variant="secondary" size="sm" onClick={() => setDeleteTarget(null)}>
              Cancel
            </Button>
            <Button variant="destructive" size="sm" onClick={handleConfirmDelete}>
              Delete
            </Button>
          </div>
        </Modal>
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
    <Card className="p-4 mb-4">
      <div className="flex items-center gap-3">
        <Input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Quest title"
          className="flex-1"
        />
        <Input
          value={slug}
          onChange={(e) => setSlug(e.target.value)}
          placeholder="slug"
          className="w-48 font-code"
        />
        <Button size="sm" onClick={() => onSubmit({ title, slug, order_index: orderIndex })}>
          Create
        </Button>
        <Button variant="ghost" size="sm" onClick={onCancel}>
          Cancel
        </Button>
      </div>
    </Card>
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
    <div className="flex items-center gap-2 px-3 py-2 bg-surface-overlay/30 rounded-lg">
      <Input
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="Level title"
        className="flex-1 h-8 text-xs"
      />
      <Input
        value={slug}
        onChange={(e) => setSlug(e.target.value)}
        placeholder="slug"
        className="w-32 h-8 text-xs font-code"
      />
      <Button
        size="sm"
        onClick={() => onSubmit({ quest_id: questId, title, slug, order_index: orderIndex })}
      >
        Add
      </Button>
      <Button variant="ghost" size="sm" onClick={onCancel}>
        Cancel
      </Button>
    </div>
  );
}
