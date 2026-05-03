import { useState, useRef, useCallback, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import Editor, { type OnMount } from '@monaco-editor/react';
import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Play,
  Send,
  RotateCcw,
  Loader2,
  CheckCircle2,
  XCircle,
  AlertTriangle,
  AlertCircle,
  GripVertical,
} from 'lucide-react';
import { useTaskDetail } from '@/hooks';
import { taskApi } from '@/services/api';
import Navbar from '@/components/layout/Navbar';
import { Button, Badge, Skeleton } from '@/components/ui';
import type { Difficulty, Task, RunResponse, SubmitResponse, TestResult } from '@/types';

const difficultyConfig: Record<
  Difficulty,
  { label: string; variant: 'success' | 'warning' | 'error' | 'info' | 'primary' }
> = {
  beginner: { label: 'Beginner', variant: 'success' },
  easy: { label: 'Easy', variant: 'info' },
  medium: { label: 'Medium', variant: 'warning' },
  hard: { label: 'Hard', variant: 'primary' },
  advanced: { label: 'Advanced', variant: 'error' },
};

type LeftTab = 'description' | 'hints' | 'submissions' | 'forum';

const leftTabs: { id: LeftTab; label: string }[] = [
  { id: 'description', label: 'Description' },
  { id: 'hints', label: 'Hints' },
  { id: 'submissions', label: 'Submissions' },
  { id: 'forum', label: 'Forum' },
];

const submissionStatusConfig: Record<
  string,
  { label: string; variant: 'success' | 'error' | 'warning' | 'info'; icon: typeof CheckCircle2 }
> = {
  passed: { label: 'Passed', variant: 'success', icon: CheckCircle2 },
  success: { label: 'Success', variant: 'success', icon: CheckCircle2 },
  failed: { label: 'Failed', variant: 'error', icon: XCircle },
  error: { label: 'Error', variant: 'warning', icon: AlertCircle },
  timeout: { label: 'Timeout', variant: 'warning', icon: AlertTriangle },
  compile_error: { label: 'Compile Error', variant: 'error', icon: XCircle },
  runtime_error: { label: 'Runtime Error', variant: 'error', icon: XCircle },
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
        <CheckCircle2 className="w-4 h-4 text-success shrink-0 mt-0.5" aria-hidden="true" />
      ) : (
        <XCircle className="w-4 h-4 text-error shrink-0 mt-0.5" aria-hidden="true" />
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
      className="pointer-events-auto bg-surface-elevated border border-success/30 rounded-lg p-4 shadow-lg min-w-[240px]"
    >
      <div className="flex items-center gap-2">
        <CheckCircle2 className="w-5 h-5 text-success" aria-hidden="true" />
        <span className="font-display font-bold text-success">+{toast.xp} XP earned!</span>
      </div>
      {toast.leveledUp && toast.newLevel && (
        <div className="mt-2 flex items-center gap-2">
          <AlertCircle className="w-5 h-5 text-warning" aria-hidden="true" />
          <span className="font-display font-bold text-warning">
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

/** Resizable divider hook for split-pane layout */
function useResizableDivider(
  initialRatio = 0.5,
  direction: 'horizontal' | 'vertical' = 'horizontal',
) {
  const [ratio, setRatio] = useState(initialRatio);
  const isDragging = useRef(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      isDragging.current = true;
      document.body.style.cursor = direction === 'horizontal' ? 'col-resize' : 'row-resize';
      document.body.style.userSelect = 'none';
    },
    [direction],
  );

  useEffect(() => {
    function handleMouseMove(e: MouseEvent) {
      if (!isDragging.current || !containerRef.current) return;
      const rect = containerRef.current.getBoundingClientRect();
      let newRatio: number;
      if (direction === 'horizontal') {
        newRatio = (e.clientX - rect.left) / rect.width;
      } else {
        newRatio = (e.clientY - rect.top) / rect.height;
      }
      // Clamp between 25% and 75%
      newRatio = Math.max(0.25, Math.min(0.75, newRatio));
      setRatio(newRatio);
    }

    function handleMouseUp() {
      isDragging.current = false;
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    }

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [direction]);

  return { ratio, containerRef, handleMouseDown };
}

function TaskSolverContent({ task }: { task: Task }) {
  const [code, setCode] = useState<string>(task.starter_code);
  const [outputState, setOutputState] = useState<OutputState>({ mode: 'idle' });
  const [activeTab, setActiveTab] = useState<LeftTab>('description');
  const [isRunning, setIsRunning] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [toasts, setToasts] = useState<XPToast[]>([]);
  const [showResults, setShowResults] = useState(true);
  const toastIdRef = useRef(0);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const editorRef = useRef<any>(null);

  const isExecuting = isRunning || isSubmitting;

  const { ratio, containerRef, handleMouseDown } = useResizableDivider(0.5, 'horizontal');

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
    setIsRunning(true);
    setOutputState({ mode: 'running' });
    setShowResults(true);

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
      setIsRunning(false);
    }
  }

  async function handleSubmit() {
    if (isExecuting) return;
    setIsSubmitting(true);
    setOutputState({ mode: 'running' });
    setShowResults(true);

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
      setIsSubmitting(false);
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
        { token: 'comment', foreground: '64748B' },
        { token: 'keyword', foreground: 'CE422B' },
        { token: 'string', foreground: '22C55E' },
        { token: 'number', foreground: 'F59E0B' },
        { token: 'type', foreground: '3B82F6' },
        { token: 'function', foreground: 'CBD5E1' },
      ],
      colors: {
        'editor.background': '#0F172A',
        'editor.foreground': '#F8FAFC',
        'editor.lineHighlightBackground': '#1E293B',
        'editor.selectionBackground': '#334155',
        'editorCursor.foreground': '#CE422B',
        'editorLineNumber.foreground': '#64748B',
        'editorLineNumber.activeForeground': '#CBD5E1',
        'editor.selectionHighlightBackground': '#334155',
        'editorIndentGuide.background': '#334155',
        'editorIndentGuide.activeBackground': '#475569',
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
    <div className="h-screen bg-surface-base flex flex-col">
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
        <div className="flex items-center gap-4 px-4 py-2.5 border-b border-border bg-surface-elevated shrink-0">
          <h1 className="font-display text-lg font-bold text-text-primary truncate">
            {task.title}
          </h1>
          <Badge variant={config.variant}>{config.label}</Badge>
          <span className="font-code text-xs text-warning font-bold shrink-0">
            +{task.xp_reward} XP
          </span>

          {/* Submission status indicator */}
          {outputState.mode === 'submit_result' && outputState.submitResult && (
            <SubmissionStatusIndicator status={outputState.submitResult.status} />
          )}
        </div>

        {/* Split pane - responsive: horizontal on md+, vertical on mobile */}
        <div ref={containerRef} className="flex-1 flex flex-col md:flex-row overflow-hidden">
          {/* Left pane - Description */}
          <div
            className="md:border-r-0 border-b md:border-b-0 border-border flex flex-col overflow-hidden"
            style={{ flex: `0 0 ${ratio * 100}%` }}
          >
            {/* Tabs */}
            <div className="flex border-b border-border shrink-0">
              {leftTabs.map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`px-4 py-2.5 text-sm font-medium transition-colors duration-200 relative cursor-pointer ${
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
                <div className="prose prose-invert prose-sm max-w-none prose-headings:font-display prose-headings:text-text-primary prose-headings:font-bold prose-h1:text-2xl prose-h2:text-xl prose-h3:text-lg prose-p:text-text-secondary prose-p:font-body prose-li:text-text-secondary prose-strong:text-text-primary prose-code:font-code prose-code:text-sm prose-code:bg-surface-overlay prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded-md prose-code:border prose-code:border-border [&_pre]:bg-surface-elevated [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg [&_pre]:p-4 [&_pre_code]:bg-transparent [&_pre_code]:border-0 [&_pre_code]:p-0 prose-a:text-primary prose-a:no-underline hover:prose-a:underline">
                  <ReactMarkdown rehypePlugins={[rehypeHighlight]}>
                    {task.description_md}
                  </ReactMarkdown>
                </div>
              )}
              {activeTab === 'hints' && (
                <div className="prose prose-invert prose-sm max-w-none prose-headings:font-display prose-headings:text-text-primary prose-code:font-code prose-code:text-sm [&_pre]:bg-surface-elevated [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg [&_pre]:p-4">
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

          {/* Resizable divider - visible only on md+ */}
          <div
            className="hidden md:flex items-center justify-center w-2 cursor-col-resize group hover:bg-primary/10 transition-colors duration-150 border-x border-border"
            onMouseDown={handleMouseDown}
            role="separator"
            aria-orientation="vertical"
            aria-label="Resize panels"
            tabIndex={0}
            onKeyDown={(e) => {
              // Allow keyboard resizing with arrow keys
              if (e.key === 'ArrowLeft') {
                e.preventDefault();
              } else if (e.key === 'ArrowRight') {
                e.preventDefault();
              }
            }}
          >
            <GripVertical
              className="w-3 h-3 text-text-muted group-hover:text-primary transition-colors duration-150"
              aria-hidden="true"
            />
          </div>

          {/* Right pane - Editor + Output */}
          <div className="flex-1 flex flex-col overflow-hidden min-h-0">
            {/* Action bar */}
            <div className="flex items-center gap-2 px-4 py-2 border-b border-border bg-surface-elevated shrink-0">
              <Button
                variant="secondary"
                size="sm"
                onClick={handleRun}
                disabled={isExecuting}
                isLoading={isRunning}
              >
                {!isRunning && <Play className="w-3.5 h-3.5" aria-hidden="true" />}
                Run
              </Button>
              <Button
                variant="primary"
                size="sm"
                onClick={handleSubmit}
                disabled={isExecuting}
                isLoading={isSubmitting}
              >
                {!isSubmitting && <Send className="w-3.5 h-3.5" aria-hidden="true" />}
                Submit
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={handleReset}
                disabled={isExecuting}
                className="ml-auto"
              >
                <RotateCcw className="w-3.5 h-3.5" aria-hidden="true" />
                Reset
              </Button>
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
                    <Loader2 className="w-5 h-5 animate-spin mr-2" aria-hidden="true" />
                    Loading editor...
                  </div>
                }
              />
            </div>

            {/* Output panel - collapsible */}
            {showResults && (
              <OutputPanel outputState={outputState} onCollapse={() => setShowResults(false)} />
            )}
            {!showResults && outputState.mode !== 'idle' && (
              <button
                onClick={() => setShowResults(true)}
                className="flex items-center gap-2 px-4 py-2 border-t border-border bg-surface-elevated text-xs text-text-muted hover:text-text-secondary transition-colors cursor-pointer"
              >
                Show output
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

function SubmissionStatusIndicator({ status }: { status: string }) {
  const config = submissionStatusConfig[status];
  if (!config) return null;

  const Icon = config.icon;
  return (
    <Badge variant={config.variant} className="ml-auto shrink-0">
      <Icon className="w-3 h-3" aria-hidden="true" />
      {config.label}
    </Badge>
  );
}

function OutputPanel({
  outputState,
  onCollapse,
}: {
  outputState: OutputState;
  onCollapse: () => void;
}) {
  return (
    <div className="h-[220px] border-t border-border bg-surface-elevated flex flex-col shrink-0">
      <div className="px-4 py-2 border-b border-border flex items-center gap-3">
        <span className="text-xs font-bold tracking-[1px] uppercase text-text-muted">Output</span>
        {outputState.mode === 'running' && (
          <Badge variant="info">
            <Loader2 className="w-3 h-3 animate-spin" aria-hidden="true" />
            Running...
          </Badge>
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
        <button
          onClick={onCollapse}
          className="ml-auto text-xs text-text-muted hover:text-text-secondary transition-colors cursor-pointer"
          aria-label="Collapse output panel"
        >
          Collapse
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        {outputState.mode === 'idle' && (
          <p className="text-sm text-text-muted">
            Click &quot;Run&quot; or &quot;Submit&quot; to see output here.
          </p>
        )}
        {outputState.mode === 'running' && (
          <div className="flex items-center gap-2 text-sm text-text-muted">
            <Loader2 className="w-4 h-4 animate-spin" aria-hidden="true" />
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
  const config = submissionStatusConfig[status];
  if (!config) {
    return <Badge variant="default">{status}</Badge>;
  }
  const Icon = config.icon;
  return (
    <Badge variant={config.variant}>
      <Icon className="w-3 h-3" aria-hidden="true" />
      {config.label}
    </Badge>
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
          <span className="text-xs font-bold text-error uppercase tracking-wide">stderr</span>
          <pre className="font-code text-sm text-error/80 whitespace-pre-wrap mt-1">
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
          <CheckCircle2 className="w-5 h-5 text-success" aria-hidden="true" />
        ) : result.status === 'failed' ? (
          <XCircle className="w-5 h-5 text-error" aria-hidden="true" />
        ) : (
          <AlertTriangle className="w-5 h-5 text-warning" aria-hidden="true" />
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
          <span className="text-xs font-bold text-error uppercase tracking-wide">stderr</span>
          <pre className="font-code text-xs text-error/80 whitespace-pre-wrap mt-1">
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
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <div className="pt-[62px] flex items-center justify-center h-[calc(100vh-62px)]">
          <div className="flex flex-col items-center gap-4">
            <Loader2 className="w-8 h-8 animate-spin text-primary" aria-hidden="true" />
            <div className="space-y-3 w-[400px]">
              <Skeleton className="h-6 w-full" />
              <Skeleton className="h-4 w-3/4" />
              <Skeleton className="h-4 w-1/2" />
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (error || !task) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <div className="pt-[62px] flex items-center justify-center h-[calc(100vh-62px)]">
          <div className="flex flex-col items-center gap-3 text-center">
            <AlertCircle className="w-8 h-8 text-error" aria-hidden="true" />
            <p className="text-text-muted">Failed to load task. Please try again later.</p>
          </div>
        </div>
      </div>
    );
  }

  return <TaskSolverContent task={task} />;
}
