import { useState, useRef, useCallback } from 'react';
import { useParams } from 'react-router-dom';
import Editor, { type OnMount } from '@monaco-editor/react';
import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import { motion, AnimatePresence } from 'framer-motion';
import { Play, Send, RotateCcw, Loader2, CheckCircle2, XCircle, AlertTriangle } from 'lucide-react';
import { useTaskDetail } from '@/hooks';
import { taskApi } from '@/services/api';
import Navbar from '@/components/layout/Navbar';
import type { Difficulty, Task, RunResponse, SubmitResponse, TestResult } from '@/types';

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

type LeftTab = 'description' | 'hints' | 'submissions' | 'forum';

const leftTabs: { id: LeftTab; label: string }[] = [
  { id: 'description', label: 'Description' },
  { id: 'hints', label: 'Hints' },
  { id: 'submissions', label: 'Submissions' },
  { id: 'forum', label: 'Forum' },
];

const statusBadgeConfig: Record<string, { label: string; className: string }> = {
  passed: { label: 'Passed', className: 'bg-green/10 text-green border-green/30' },
  failed: { label: 'Failed', className: 'bg-red-400/10 text-red-400 border-red-400/30' },
  error: { label: 'Error', className: 'bg-amber/10 text-amber border-amber/30' },
  timeout: { label: 'Timeout', className: 'bg-amber/10 text-amber border-amber/30' },
  success: { label: 'Success', className: 'bg-green/10 text-green border-green/30' },
  compile_error: {
    label: 'Compile Error',
    className: 'bg-red-400/10 text-red-400 border-red-400/30',
  },
  runtime_error: {
    label: 'Runtime Error',
    className: 'bg-red-400/10 text-red-400 border-red-400/30',
  },
};

interface XPToast {
  id: number;
  xp: number;
  leveledUp: boolean;
  newLevel: number | null;
}

function TestResultItem({ result }: { result: TestResult }) {
  return (
    <div className="flex items-start gap-2 py-1.5">
      {result.passed ? (
        <CheckCircle2 className="w-4 h-4 text-green shrink-0 mt-0.5" />
      ) : (
        <XCircle className="w-4 h-4 text-red-400 shrink-0 mt-0.5" />
      )}
      <div className="flex-1 min-w-0">
        <span className="font-code text-sm text-text-primary">{result.name}</span>
        {result.message && (
          <p className="font-code text-xs text-text-muted mt-0.5 whitespace-pre-wrap">
            {result.message}
          </p>
        )}
      </div>
    </div>
  );
}

function XPToastNotification({ toast, onDismiss }: { toast: XPToast; onDismiss: () => void }) {
  return (
    <motion.div
      initial={{ opacity: 0, x: 100, y: 0 }}
      animate={{ opacity: 1, x: 0, y: 0 }}
      exit={{ opacity: 0, x: 100 }}
      transition={{ type: 'spring', stiffness: 300, damping: 25 }}
      onAnimationComplete={(definition) => {
        if (definition === 'exit') onDismiss();
      }}
      className="pointer-events-auto bg-dark-800 border border-green/30 rounded-lg p-4 shadow-lg min-w-[240px]"
    >
      <div className="flex items-center gap-2">
        <span className="text-lg">🎉</span>
        <span className="font-display font-bold text-green">+{toast.xp} XP earned!</span>
      </div>
      {toast.leveledUp && toast.newLevel && (
        <div className="mt-2 flex items-center gap-2">
          <span className="text-lg">🚀</span>
          <span className="font-display font-bold text-amber">
            Level Up! You&apos;re now Level {toast.newLevel}
          </span>
        </div>
      )}
    </motion.div>
  );
}

type OutputMode = 'idle' | 'running' | 'run_result' | 'submit_result';

interface OutputState {
  mode: OutputMode;
  runResult?: RunResponse;
  submitResult?: SubmitResponse;
}

function TaskSolverContent({ task }: { task: Task }) {
  const [code, setCode] = useState<string>(task.starter_code);
  const [outputState, setOutputState] = useState<OutputState>({ mode: 'idle' });
  const [activeTab, setActiveTab] = useState<LeftTab>('description');
  const [isExecuting, setIsExecuting] = useState(false);
  const [toasts, setToasts] = useState<XPToast[]>([]);
  const toastIdRef = useRef(0);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const editorRef = useRef<any>(null);

  const addXPToast = useCallback((xp: number, leveledUp: boolean, newLevel: number | null) => {
    const id = ++toastIdRef.current;
    setToasts((prev) => [...prev, { id, xp, leveledUp, newLevel }]);
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 4000);
  }, []);

  const removeToast = useCallback((id: number) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  async function handleRun() {
    if (isExecuting) return;
    setIsExecuting(true);
    setOutputState({ mode: 'running' });

    try {
      const result = await taskApi.run(task.slug, code);
      setOutputState({ mode: 'run_result', runResult: result });
    } catch (error: unknown) {
      const err = error as { message?: string };
      setOutputState({
        mode: 'run_result',
        runResult: {
          status: 'runtime_error',
          stdout: '',
          stderr: err.message || 'An unexpected error occurred',
          duration_ms: 0,
        },
      });
    } finally {
      setIsExecuting(false);
    }
  }

  async function handleSubmit() {
    if (isExecuting) return;
    setIsExecuting(true);
    setOutputState({ mode: 'running' });

    try {
      const result = await taskApi.submit(task.slug, code);
      setOutputState({ mode: 'submit_result', submitResult: result });

      if (result.status === 'passed' && result.xp_awarded > 0) {
        addXPToast(result.xp_awarded, result.leveled_up, result.new_level);
      }
    } catch (error: unknown) {
      const err = error as { message?: string };
      setOutputState({
        mode: 'run_result',
        runResult: {
          status: 'runtime_error',
          stdout: '',
          stderr: err.message || 'An unexpected error occurred',
          duration_ms: 0,
        },
      });
    } finally {
      setIsExecuting(false);
    }
  }

  function handleReset() {
    setCode(task.starter_code);
    editorRef.current?.setValue(task.starter_code);
    setOutputState({ mode: 'idle' });
  }

  const handleEditorMount: OnMount = (editorInstance, monaco) => {
    editorRef.current = editorInstance;

    monaco.editor.defineTheme('rustgym-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '756F68' },
        { token: 'keyword', foreground: 'CE422B' },
        { token: 'string', foreground: '4ADE80' },
        { token: 'number', foreground: 'E8913A' },
        { token: 'type', foreground: '60A5FA' },
      ],
      colors: {
        'editor.background': '#111110',
        'editor.foreground': '#F0EDE8',
        'editor.lineHighlightBackground': '#1A1917',
        'editor.selectionBackground': '#2E2D2A',
        'editorCursor.foreground': '#CE422B',
        'editorLineNumber.foreground': '#756F68',
        'editorLineNumber.activeForeground': '#B8B3AC',
      },
    });
    monaco.editor.setTheme('rustgym-dark');

    editorInstance.addAction({
      id: 'submit-code',
      label: 'Submit Code',
      keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter],
      run: () => {
        handleSubmit();
      },
    });
  };

  const config = difficultyConfig[task.difficulty];

  return (
    <div className="h-screen bg-dark-950 flex flex-col">
      <Navbar />

      {/* XP Toast notifications */}
      <div className="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
        <AnimatePresence>
          {toasts.map((toast) => (
            <XPToastNotification
              key={toast.id}
              toast={toast}
              onDismiss={() => removeToast(toast.id)}
            />
          ))}
        </AnimatePresence>
      </div>

      <div className="pt-[62px] flex-1 flex flex-col overflow-hidden">
        {/* Task header bar */}
        <div className="flex items-center gap-4 px-4 py-2.5 border-b border-border bg-dark-900 shrink-0">
          <h1 className="font-display text-lg font-bold text-text-primary truncate">
            {task.title}
          </h1>
          <span
            className={`text-[11px] font-bold px-2.5 py-0.5 rounded-full border shrink-0 ${config.className}`}
          >
            {config.label}
          </span>
          <span className="font-code text-xs text-amber font-bold shrink-0">
            +{task.xp_reward} XP
          </span>
        </div>

        {/* Split pane */}
        <div className="flex-1 flex overflow-hidden">
          {/* Left pane - Description */}
          <div className="w-1/2 border-r border-border flex flex-col overflow-hidden">
            {/* Tabs */}
            <div className="flex border-b border-border shrink-0">
              {leftTabs.map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`px-4 py-2.5 text-sm font-medium transition-colors relative ${
                    activeTab === tab.id
                      ? 'text-text-primary'
                      : 'text-text-muted hover:text-text-secondary'
                  }`}
                >
                  {tab.label}
                  {activeTab === tab.id && (
                    <span className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />
                  )}
                </button>
              ))}
            </div>

            {/* Tab content */}
            <div className="flex-1 overflow-y-auto p-6">
              {activeTab === 'description' && (
                <div className="prose prose-invert prose-sm max-w-none [&_pre]:bg-dark-800 [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg [&_code]:font-code [&_code]:text-sm [&_h1]:font-display [&_h2]:font-display [&_h3]:font-display">
                  <ReactMarkdown rehypePlugins={[rehypeHighlight]}>
                    {task.description_md}
                  </ReactMarkdown>
                </div>
              )}
              {activeTab === 'hints' && (
                <div className="prose prose-invert prose-sm max-w-none [&_pre]:bg-dark-800 [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg [&_code]:font-code [&_code]:text-sm">
                  {task.hint_md ? (
                    <ReactMarkdown rehypePlugins={[rehypeHighlight]}>{task.hint_md}</ReactMarkdown>
                  ) : (
                    <p className="text-text-muted">No hints available for this task.</p>
                  )}
                </div>
              )}
              {activeTab === 'submissions' && (
                <p className="text-text-muted">Your submissions will appear here.</p>
              )}
              {activeTab === 'forum' && (
                <p className="text-text-muted">Forum discussions for this task.</p>
              )}
            </div>
          </div>

          {/* Right pane - Editor + Output */}
          <div className="w-1/2 flex flex-col overflow-hidden">
            {/* Action bar */}
            <div className="flex items-center gap-2 px-4 py-2 border-b border-border bg-dark-900 shrink-0">
              <button
                onClick={handleRun}
                disabled={isExecuting}
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-text-primary bg-dark-700 border border-border rounded-lg hover:border-border-light transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isExecuting ? (
                  <Loader2 className="w-3.5 h-3.5 animate-spin" />
                ) : (
                  <Play className="w-3.5 h-3.5" />
                )}
                Run
              </button>
              <button
                onClick={handleSubmit}
                disabled={isExecuting}
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-white bg-primary rounded-lg hover:bg-primary-light transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isExecuting ? (
                  <Loader2 className="w-3.5 h-3.5 animate-spin" />
                ) : (
                  <Send className="w-3.5 h-3.5" />
                )}
                Submit
              </button>
              <button
                onClick={handleReset}
                disabled={isExecuting}
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-text-muted hover:text-text-primary transition-colors ml-auto disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <RotateCcw className="w-3.5 h-3.5" />
                Reset
              </button>
            </div>

            {/* Monaco Editor */}
            <div className="flex-1 min-h-0">
              <Editor
                height="100%"
                language="rust"
                value={code}
                onChange={(value) => setCode(value ?? '')}
                onMount={handleEditorMount}
                options={{
                  fontFamily: "'JetBrains Mono', monospace",
                  fontSize: 14,
                  tabSize: 4,
                  minimap: { enabled: false },
                  lineNumbers: 'on',
                  scrollBeyondLastLine: false,
                  automaticLayout: true,
                  padding: { top: 12 },
                }}
                loading={
                  <div className="flex items-center justify-center h-full text-text-muted">
                    Loading editor...
                  </div>
                }
              />
            </div>

            {/* Output panel */}
            <OutputPanel outputState={outputState} />
          </div>
        </div>
      </div>
    </div>
  );
}

function OutputPanel({ outputState }: { outputState: OutputState }) {
  return (
    <div className="h-[220px] border-t border-border bg-dark-900 flex flex-col shrink-0">
      <div className="px-4 py-2 border-b border-border flex items-center gap-3">
        <span className="text-xs font-bold tracking-[1px] uppercase text-text-muted">Output</span>
        {outputState.mode === 'running' && (
          <span className="flex items-center gap-1.5 text-xs text-text-muted">
            <Loader2 className="w-3 h-3 animate-spin" />
            Running...
          </span>
        )}
        {outputState.mode === 'run_result' && outputState.runResult && (
          <OutputStatusBadge status={outputState.runResult.status} />
        )}
        {outputState.mode === 'submit_result' && outputState.submitResult && (
          <>
            <OutputStatusBadge status={outputState.submitResult.status} />
            <span className="text-xs text-text-muted ml-auto">
              {outputState.submitResult.duration_ms}ms · {outputState.submitResult.memory_kb}KB
            </span>
          </>
        )}
        {outputState.mode === 'run_result' &&
          outputState.runResult &&
          outputState.runResult.duration_ms > 0 && (
            <span className="text-xs text-text-muted ml-auto">
              {outputState.runResult.duration_ms}ms
            </span>
          )}
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        {outputState.mode === 'idle' && (
          <p className="text-sm text-text-muted">
            Click &quot;Run&quot; or &quot;Submit&quot; to see output here.
          </p>
        )}
        {outputState.mode === 'running' && (
          <div className="flex items-center gap-2 text-sm text-text-muted">
            <Loader2 className="w-4 h-4 animate-spin" />
            Executing your code...
          </div>
        )}
        {outputState.mode === 'run_result' && outputState.runResult && (
          <RunResultView result={outputState.runResult} />
        )}
        {outputState.mode === 'submit_result' && outputState.submitResult && (
          <SubmitResultView result={outputState.submitResult} />
        )}
      </div>
    </div>
  );
}

function OutputStatusBadge({ status }: { status: string }) {
  const badge = statusBadgeConfig[status] || {
    label: status,
    className: 'bg-dark-700 text-text-muted border-border',
  };
  return (
    <span className={`text-[11px] font-bold px-2 py-0.5 rounded-full border ${badge.className}`}>
      {badge.label}
    </span>
  );
}

function RunResultView({ result }: { result: RunResponse }) {
  return (
    <div className="space-y-2">
      {result.stdout && (
        <div>
          <span className="text-xs font-bold text-text-muted uppercase tracking-wide">stdout</span>
          <pre className="font-code text-sm text-text-secondary whitespace-pre-wrap mt-1">
            {result.stdout}
          </pre>
        </div>
      )}
      {result.stderr && (
        <div>
          <span className="text-xs font-bold text-red-400 uppercase tracking-wide">stderr</span>
          <pre className="font-code text-sm text-red-400/80 whitespace-pre-wrap mt-1">
            {result.stderr}
          </pre>
        </div>
      )}
      {!result.stdout && !result.stderr && (
        <p className="text-sm text-text-muted">No output produced.</p>
      )}
    </div>
  );
}

function SubmitResultView({ result }: { result: SubmitResponse }) {
  const passedCount = result.test_results.filter((t) => t.passed).length;
  const totalCount = result.test_results.length;

  return (
    <div className="space-y-3">
      {/* Summary */}
      <div className="flex items-center gap-3">
        {result.status === 'passed' ? (
          <CheckCircle2 className="w-5 h-5 text-green" />
        ) : result.status === 'failed' ? (
          <XCircle className="w-5 h-5 text-red-400" />
        ) : (
          <AlertTriangle className="w-5 h-5 text-amber" />
        )}
        <span className="text-sm font-medium text-text-primary">
          {passedCount}/{totalCount} tests passed
        </span>
      </div>

      {/* Test results list */}
      {result.test_results.length > 0 && (
        <div className="space-y-0.5">
          {result.test_results.map((test, i) => (
            <TestResultItem key={i} result={test} />
          ))}
        </div>
      )}

      {/* stdout/stderr */}
      {result.stdout && (
        <div className="pt-2 border-t border-border">
          <span className="text-xs font-bold text-text-muted uppercase tracking-wide">stdout</span>
          <pre className="font-code text-xs text-text-secondary whitespace-pre-wrap mt-1">
            {result.stdout}
          </pre>
        </div>
      )}
      {result.stderr && (
        <div className="pt-2 border-t border-border">
          <span className="text-xs font-bold text-red-400 uppercase tracking-wide">stderr</span>
          <pre className="font-code text-xs text-red-400/80 whitespace-pre-wrap mt-1">
            {result.stderr}
          </pre>
        </div>
      )}
    </div>
  );
}

export default function TaskSolver() {
  const { slug } = useParams<{ slug: string }>();
  const { data: task, isLoading, error } = useTaskDetail(slug ?? '');

  if (isLoading) {
    return (
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <div className="pt-[62px] flex items-center justify-center h-[calc(100vh-62px)]">
          <div className="flex flex-col items-center gap-4">
            <Loader2 className="w-8 h-8 animate-spin text-primary" />
            <div className="space-y-3 w-[400px]">
              <div className="h-6 bg-dark-800 rounded animate-pulse" />
              <div className="h-4 bg-dark-800 rounded animate-pulse w-3/4" />
              <div className="h-4 bg-dark-800 rounded animate-pulse w-1/2" />
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (error || !task) {
    return (
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <div className="pt-[62px] flex items-center justify-center h-[calc(100vh-62px)]">
          <p className="text-text-muted">Failed to load task. Please try again later.</p>
        </div>
      </div>
    );
  }

  return <TaskSolverContent task={task} />;
}
