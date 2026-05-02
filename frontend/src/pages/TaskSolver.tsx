import { useState, useRef } from 'react';
import { useParams } from 'react-router-dom';
import Editor, { type OnMount } from '@monaco-editor/react';
import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import { Play, Send, RotateCcw } from 'lucide-react';
import { useTaskDetail } from '@/hooks';
import Navbar from '@/components/layout/Navbar';
import type { Difficulty, Task } from '@/types';

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

function TaskSolverContent({ task }: { task: Task }) {
  const [code, setCode] = useState<string>(task.starter_code);
  const [output, setOutput] = useState<string>('');
  const [activeTab, setActiveTab] = useState<LeftTab>('description');
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const editorRef = useRef<any>(null);

  function handleRun() {
    setOutput(
      `[Run] Executing code...\n\n// Output will appear here when the runner is connected.`,
    );
    console.log('[RustGym] Run triggered with code:', code.slice(0, 100) + '...');
  }

  function handleSubmit() {
    setOutput(
      `[Submit] Submitting code for grading...\n\n// Results will appear here when the runner is connected.`,
    );
    console.log('[RustGym] Submit triggered with code:', code.slice(0, 100) + '...');
  }

  function handleReset() {
    setCode(task.starter_code);
    editorRef.current?.setValue(task.starter_code);
    setOutput('');
  }

  const handleEditorMount: OnMount = (editorInstance, monaco) => {
    editorRef.current = editorInstance;

    // Define custom dark theme
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

    // Add Ctrl+Enter shortcut for submit
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
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-text-primary bg-dark-700 border border-border rounded-lg hover:border-border-light transition-colors"
              >
                <Play className="w-3.5 h-3.5" />
                Run
              </button>
              <button
                onClick={handleSubmit}
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-white bg-primary rounded-lg hover:bg-primary-light transition-colors"
              >
                <Send className="w-3.5 h-3.5" />
                Submit
              </button>
              <button
                onClick={handleReset}
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-text-muted hover:text-text-primary transition-colors ml-auto"
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
            <div className="h-[180px] border-t border-border bg-dark-900 flex flex-col shrink-0">
              <div className="px-4 py-2 border-b border-border">
                <span className="text-xs font-bold tracking-[1px] uppercase text-text-muted">
                  Output
                </span>
              </div>
              <div className="flex-1 overflow-y-auto p-4">
                {output ? (
                  <pre className="font-code text-sm text-text-secondary whitespace-pre-wrap">
                    {output}
                  </pre>
                ) : (
                  <p className="text-sm text-text-muted">
                    Click &quot;Run&quot; or &quot;Submit&quot; to see output here.
                  </p>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
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
          <div className="animate-pulse text-text-muted">Loading task...</div>
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
