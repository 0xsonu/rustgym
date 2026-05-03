import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Plus, Pencil, Trash2, Save, X } from 'lucide-react';
import Editor from '@monaco-editor/react';
import { adminApi } from '@/services/adminApi';
import type { AdminTask } from '@/types/admin';
import { Button, Input, Card } from '@/components/ui';
import AdminLayout from './AdminLayout';

type EditingTask = Partial<AdminTask> & { isNew?: boolean };

export default function ChallengeEditor() {
  const queryClient = useQueryClient();
  const [editing, setEditing] = useState<EditingTask | null>(null);
  const [activeTab, setActiveTab] = useState<'starter' | 'solution' | 'test' | 'cargo'>('starter');

  const { data: tasks, isLoading } = useQuery({
    queryKey: ['admin', 'tasks'],
    queryFn: () => adminApi.listTasks(),
  });

  const createMutation = useMutation({
    mutationFn: (data: Partial<AdminTask>) => adminApi.createTask(data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'tasks'] });
      setEditing(null);
    },
  });

  const updateMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<AdminTask> }) =>
      adminApi.updateTask(id, data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'tasks'] });
      setEditing(null);
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => adminApi.deleteTask(id),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'tasks'] });
    },
  });

  function handleSave() {
    if (!editing) return;
    if (editing.isNew) {
      createMutation.mutate(editing);
    } else if (editing.id) {
      updateMutation.mutate({ id: editing.id, data: editing });
    }
  }

  function handleNew() {
    setEditing({
      isNew: true,
      slug: '',
      title: '',
      description_md: '',
      starter_code: '// Write your solution here\n',
      solution_code: '',
      test_code: '',
      cargo_toml: '[package]\nname = "task"\nversion = "0.1.0"\nedition = "2021"\n',
      difficulty: 'easy',
      xp_reward: 50,
      order_index: (tasks?.length ?? 0) + 1,
      is_published: false,
      tags: [],
      syntest_rules: [],
    });
    setActiveTab('starter');
  }

  return (
    <AdminLayout>
      <div className="flex flex-col h-[calc(100vh-64px)]">
        <div className="flex items-center justify-between mb-4">
          <h1 className="font-display text-2xl font-bold text-text-primary">Challenge Editor</h1>
          <Button onClick={handleNew} size="md">
            <Plus className="w-4 h-4" />
            New Task
          </Button>
        </div>

        <div className="flex gap-4 flex-1 min-h-0">
          {/* Task List */}
          <Card className="w-72 shrink-0 p-0 overflow-auto">
            {isLoading ? (
              <div className="p-4 text-sm text-text-muted font-body">Loading...</div>
            ) : (
              <div className="p-2 space-y-1">
                {tasks?.map((task) => (
                  <div
                    key={task.id}
                    className={`flex items-center justify-between px-3 py-2 rounded-lg cursor-pointer transition-colors duration-200 ${
                      editing?.id === task.id
                        ? 'bg-primary/10 border border-primary/20'
                        : 'hover:bg-surface-overlay'
                    }`}
                    onClick={() => {
                      setEditing({ ...task });
                      setActiveTab('starter');
                    }}
                  >
                    <div className="min-w-0 flex-1">
                      <p className="text-sm text-text-primary truncate font-body">{task.title}</p>
                      <p className="text-[11px] text-text-muted font-body">{task.difficulty}</p>
                    </div>
                    <div className="flex items-center gap-1 ml-2">
                      <span
                        className={`w-2 h-2 rounded-full ${task.is_published ? 'bg-success' : 'bg-text-muted'}`}
                      />
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          if (confirm('Delete this task?')) {
                            deleteMutation.mutate(task.id);
                          }
                        }}
                        className="p-1 text-text-muted hover:text-error transition-colors duration-200 cursor-pointer"
                        aria-label="Delete task"
                      >
                        <Trash2 className="w-3 h-3" />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </Card>

          {/* Editor Panel */}
          {editing ? (
            <Card className="flex-1 p-0 flex flex-col min-h-0">
              {/* Task metadata */}
              <div className="p-4 border-b border-border space-y-3">
                <div className="flex items-center gap-3">
                  <Input
                    value={editing.title ?? ''}
                    onChange={(e) => setEditing({ ...editing, title: e.target.value })}
                    placeholder="Task title"
                    className="flex-1"
                  />
                  <Input
                    value={editing.slug ?? ''}
                    onChange={(e) => setEditing({ ...editing, slug: e.target.value })}
                    placeholder="slug"
                    className="w-48 font-code"
                  />
                </div>
                <div className="flex items-center gap-3">
                  <select
                    value={editing.difficulty ?? 'easy'}
                    onChange={(e) => setEditing({ ...editing, difficulty: e.target.value })}
                    className="h-10 bg-surface-base border border-border rounded-lg px-3 py-2 text-sm text-text-primary font-body focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary"
                  >
                    <option value="beginner">Beginner</option>
                    <option value="easy">Easy</option>
                    <option value="medium">Medium</option>
                    <option value="hard">Hard</option>
                    <option value="advanced">Advanced</option>
                  </select>
                  <Input
                    type="number"
                    value={editing.xp_reward ?? 50}
                    onChange={(e) =>
                      setEditing({ ...editing, xp_reward: parseInt(e.target.value) || 0 })
                    }
                    placeholder="XP"
                    className="w-24"
                  />
                  <label className="flex items-center gap-2 text-sm text-text-secondary font-body">
                    <input
                      type="checkbox"
                      checked={editing.is_published ?? false}
                      onChange={(e) => setEditing({ ...editing, is_published: e.target.checked })}
                      className="rounded border-border"
                    />
                    Published
                  </label>
                  <div className="ml-auto flex items-center gap-2">
                    <Button variant="ghost" size="sm" onClick={() => setEditing(null)}>
                      <X className="w-4 h-4" />
                      Cancel
                    </Button>
                    <Button
                      size="sm"
                      onClick={handleSave}
                      disabled={createMutation.isPending || updateMutation.isPending}
                      isLoading={createMutation.isPending || updateMutation.isPending}
                    >
                      {editing.isNew ? <Plus className="w-4 h-4" /> : <Save className="w-4 h-4" />}
                      {editing.isNew ? 'Create' : 'Save'}
                    </Button>
                  </div>
                </div>
              </div>

              {/* Code tabs */}
              <div className="flex border-b border-border">
                {(
                  [
                    ['starter', 'Starter Code'],
                    ['solution', 'Solution'],
                    ['test', 'Tests'],
                    ['cargo', 'Cargo.toml'],
                  ] as const
                ).map(([key, label]) => (
                  <button
                    key={key}
                    onClick={() => setActiveTab(key)}
                    className={`px-4 py-2.5 text-sm font-medium transition-colors duration-200 border-b-2 cursor-pointer ${
                      activeTab === key
                        ? 'text-primary border-primary'
                        : 'text-text-muted border-transparent hover:text-text-primary'
                    }`}
                  >
                    {label}
                  </button>
                ))}
              </div>

              {/* Monaco Editor */}
              <div className="flex-1 min-h-0">
                <Editor
                  height="100%"
                  language={activeTab === 'cargo' ? 'toml' : 'rust'}
                  theme="vs-dark"
                  value={getCodeValue(editing, activeTab)}
                  onChange={(value) =>
                    setEditing({ ...editing, ...setCodeValue(activeTab, value ?? '') })
                  }
                  options={{
                    minimap: { enabled: false },
                    fontSize: 13,
                    lineNumbers: 'on',
                    scrollBeyondLastLine: false,
                    wordWrap: 'on',
                  }}
                />
              </div>
            </Card>
          ) : (
            <Card className="flex-1 flex items-center justify-center">
              <div className="text-center">
                <Pencil className="w-8 h-8 text-text-muted mx-auto mb-3" />
                <p className="text-sm text-text-muted font-body">
                  Select a task to edit or create a new one
                </p>
              </div>
            </Card>
          )}
        </div>
      </div>
    </AdminLayout>
  );
}

function getCodeValue(task: EditingTask, tab: 'starter' | 'solution' | 'test' | 'cargo'): string {
  switch (tab) {
    case 'starter':
      return task.starter_code ?? '';
    case 'solution':
      return task.solution_code ?? '';
    case 'test':
      return task.test_code ?? '';
    case 'cargo':
      return task.cargo_toml ?? '';
  }
}

function setCodeValue(
  tab: 'starter' | 'solution' | 'test' | 'cargo',
  value: string,
): Partial<AdminTask> {
  switch (tab) {
    case 'starter':
      return { starter_code: value };
    case 'solution':
      return { solution_code: value };
    case 'test':
      return { test_code: value };
    case 'cargo':
      return { cargo_toml: value };
  }
}
