import { FadeIn } from '../Landing';

const steps = [
  {
    num: '01',
    title: 'Start',
    desc: 'Complete a quick survey and get a personalized Rust learning plan matched to your background and goals.',
  },
  {
    num: '02',
    title: 'Learn',
    desc: 'Read concise, digestible lessons on Rust theory — ownership, traits, lifetimes, async, and more.',
  },
  {
    num: '03',
    title: 'Practice',
    desc: 'Write Rust code in our in-browser IDE from lesson one. Get instant, borrow-checker-aware feedback on every task.',
  },
  {
    num: '04',
    title: 'Create',
    desc: 'Build real-world projects — a CLI tool, a web server, a WebAssembly module — and add them to your portfolio.',
  },
];

const quests = [
  {
    icon: '✅',
    title: 'Rust Syntax Basics',
    info: '15 levels · 120 tasks',
    status: 'Done',
    statusClass: 'bg-green/10 text-green',
  },
  {
    icon: '🔥',
    title: 'Ownership & Borrowing',
    info: 'Level 7/18 · 64 tasks',
    status: 'Active',
    statusClass: 'bg-primary/[0.15] text-primary-light',
    active: true,
  },
  {
    icon: '🔒',
    title: 'Lifetimes & Traits',
    info: '20 levels · 140 tasks',
    status: 'Locked',
    statusClass: 'bg-dark-950 text-text-muted',
  },
  {
    icon: '🔒',
    title: 'Async & Concurrency',
    info: '22 levels · 160 tasks',
    status: 'Locked',
    statusClass: 'bg-dark-950 text-text-muted',
  },
];

export default function AboutSection() {
  return (
    <section className="py-24 px-5 md:px-10 bg-dark-900">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          About the course
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          What is learning <span className="text-primary">Rust with RustGym</span> like?
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed">
          A gamified coding adventure that takes you from zero to systems-level Rust — through
          quests, tasks, and real projects.
        </p>

        <FadeIn>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-20 items-center mt-16">
            {/* Steps */}
            <div className="flex flex-col">
              {steps.map((step, i) => (
                <div
                  key={step.num}
                  className={`flex gap-5 py-6 ${i < steps.length - 1 ? 'border-b border-border' : ''}`}
                >
                  <div className="w-10 h-10 rounded-[10px] bg-primary/[0.12] border border-primary/25 flex items-center justify-center font-code text-[13px] font-bold text-primary shrink-0">
                    {step.num}
                  </div>
                  <div>
                    <h3 className="font-display text-xl font-bold mb-1">{step.title}</h3>
                    <p className="text-sm text-text-secondary leading-relaxed">{step.desc}</p>
                  </div>
                </div>
              ))}
            </div>

            {/* Visual */}
            <div className="bg-dark-card border border-border rounded-[20px] p-8 relative overflow-hidden">
              <div className="inline-flex items-center gap-2 bg-primary/10 border border-primary/30 px-3.5 py-1.5 rounded-full font-code text-xs font-bold text-primary-light mb-5">
                🦀 Level 12 — Rustacean
              </div>
              <div className="mb-6">
                <div className="flex justify-between text-xs text-text-muted mb-1.5 font-code">
                  <span>XP Progress</span>
                  <span>7,240 / 10,000</span>
                </div>
                <div className="h-2 bg-dark-700 rounded overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-primary to-amber rounded"
                    style={{ width: '72%' }}
                  />
                </div>
              </div>
              <div className="text-xs text-text-muted mb-4 font-code">ACTIVE QUESTS</div>
              <div className="flex flex-col gap-2.5">
                {quests.map((q) => (
                  <div
                    key={q.title}
                    className={`flex items-center gap-3.5 bg-dark-700 border rounded-[10px] px-4 py-3 transition-colors ${q.active ? 'border-primary' : 'border-border'}`}
                  >
                    <div className="text-xl">{q.icon}</div>
                    <div>
                      <h4 className="text-sm font-semibold mb-0.5">{q.title}</h4>
                      <p className="text-xs text-text-muted font-code">{q.info}</p>
                    </div>
                    <span
                      className={`ml-auto text-[11px] px-2.5 py-0.5 rounded-[10px] font-semibold ${q.statusClass}`}
                    >
                      {q.status}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
