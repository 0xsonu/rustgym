import { FadeIn } from '../Landing';

const features = [
  {
    icon: '⚡',
    title: '80% Practice, 20% Theory',
    desc: 'Rust is learned by writing Rust. Our 1,400+ coding tasks start from your very first lesson. The borrow checker becomes your friend, not your enemy — through repetition, not reading.',
  },
  {
    icon: '🎮',
    title: 'Gamified Learning Journey',
    desc: 'Earn XP, unlock quests, level up your Rustacean rank, and collect achievements. Your progress is saved automatically — pick up exactly where you left off, on any device.',
  },
  {
    icon: '🧠',
    title: 'AI-Powered Code Review',
    desc: 'Every solution gets reviewed by an AI mentor trained on idiomatic Rust. Get personalized tips on performance, safety, and style — not just "wrong" or "right".',
  },
  {
    icon: '🗺️',
    title: 'Structured Quest Roadmap',
    desc: 'Your curriculum is broken into focused quests — Syntax, Ownership, Traits, Async, Systems, WebAssembly, and more. Each quest is a complete module, sized for real-life schedules.',
  },
];

export default function FeaturesSection() {
  return (
    <section className="py-24 px-5 md:px-10">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          Features
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          Why learn <span className="text-primary">Rust with RustGym</span>
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed">
          Everything you need to go from curious beginner to confident systems programmer — without
          the usual frustration.
        </p>

        <FadeIn>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-16">
            {features.map((f) => (
              <div
                key={f.title}
                className="group bg-dark-card border border-border rounded-2xl p-8 transition-all relative overflow-hidden hover:border-border-light hover:-translate-y-[3px]"
              >
                <div className="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-primary to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
                <div className="w-12 h-12 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-[22px] mb-5">
                  {f.icon}
                </div>
                <h3 className="font-display text-xl font-bold mb-2.5">{f.title}</h3>
                <p className="text-sm text-text-secondary leading-relaxed">{f.desc}</p>
              </div>
            ))}
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
