import { FadeIn } from '../Landing';

const quests = [
  {
    level: 'QUEST 1 · 15 levels',
    name: 'Rust Foundations',
    tasks: '120 tasks · Beginner',
    progress: 100,
    active: true,
  },
  {
    level: 'QUEST 2 · 18 levels',
    name: 'Ownership & Borrowing',
    tasks: '140 tasks · Core Rust',
    progress: 40,
  },
  {
    level: 'QUEST 3 · 20 levels',
    name: 'Traits & Generics',
    tasks: '160 tasks · Intermediate',
    progress: 0,
  },
  {
    level: 'QUEST 4 · 22 levels',
    name: 'Error Handling',
    tasks: '140 tasks · Intermediate',
    progress: 0,
  },
  {
    level: 'QUEST 5 · 25 levels',
    name: 'Async & Concurrency',
    tasks: '200 tasks · Advanced',
    progress: 0,
  },
  {
    level: 'QUEST 6 · 20 levels',
    name: 'Systems Programming',
    tasks: '180 tasks · Advanced',
    progress: 0,
  },
  {
    level: 'QUEST 7 · 15 levels',
    name: 'WebAssembly & Projects',
    tasks: '160 tasks · Expert',
    progress: 0,
  },
];

export default function QuestMapSection() {
  return (
    <section className="py-24 px-5 md:px-10">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          Quest Map
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          Your Rust <span className="text-primary">Learning Path</span>
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed font-body">
          7 epic quests, 90+ levels, 1,400+ tasks. From &ldquo;Hello, Ferris!&rdquo; to deploying
          production Rust.
        </p>

        <FadeIn>
          <div className="flex gap-4 mt-12 overflow-x-auto pb-4 scrollbar-thin">
            {quests.map((q) => (
              <div
                key={q.name}
                className={`shrink-0 bg-surface-elevated border rounded-[14px] px-6 py-5 min-w-[200px] transition-all hover:border-primary hover:-translate-y-[3px] cursor-pointer ${
                  q.active ? 'border-primary bg-primary/[0.05]' : 'border-border'
                }`}
              >
                <div className="font-code text-[11px] text-text-muted mb-1.5">{q.level}</div>
                <div className="font-display text-lg font-bold mb-1">{q.name}</div>
                <div className="text-xs text-text-muted font-body">{q.tasks}</div>
                <div className="h-[3px] bg-slate-800 rounded mt-3.5 overflow-hidden">
                  <div className="h-full bg-primary rounded" style={{ width: `${q.progress}%` }} />
                </div>
              </div>
            ))}
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
