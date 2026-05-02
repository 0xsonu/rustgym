import { FadeIn } from '../Landing';

const reviews = [
  {
    text: "I'd tried the Rust Book three times and bounced off ownership every time. RustGym's task-based approach finally made it click. By task 50 I wasn't fighting the borrow checker — I was thinking like it.",
    name: 'Arjun Kapoor',
    role: '🇮🇳 India',
    job: 'Systems Engineer at Cloudflare',
    initials: 'AK',
    avatarBg: 'bg-primary/[0.15]',
    avatarColor: 'text-primary-light',
  },
  {
    text: 'The gamification kept me going when motivation dipped. Unlocking the "Lifetime Whisperer" achievement felt genuinely satisfying. Six months later I shipped production Rust at my job.',
    name: 'Sophie Richter',
    role: '🇩🇪 Germany',
    job: 'Rust Developer at Ditto',
    initials: 'SR',
    avatarBg: 'bg-blue/[0.12]',
    avatarColor: 'text-blue',
  },
  {
    text: 'I came from Python with zero systems background. The quest structure is perfectly paced. The in-browser IDE means no setup friction. I passed my first Rust interview after 4 months on RustGym.',
    name: 'Marcus Lee',
    role: '🇸🇬 Singapore',
    job: 'Backend Engineer at Grab',
    initials: 'ML',
    avatarBg: 'bg-green/10',
    avatarColor: 'text-green',
  },
];

export default function ReviewsSection() {
  return (
    <section className="py-24 px-5 md:px-10 bg-dark-900">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          Reviews
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          From learners, <span className="text-primary">for learners</span>
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed">
          Real stories from developers who went from Rust-curious to Rust-employed.
        </p>

        <FadeIn>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-16">
            {reviews.map((r) => (
              <div
                key={r.name}
                className="bg-dark-card border border-border rounded-2xl p-7 flex flex-col gap-4 transition-colors hover:border-border-light"
              >
                <div className="text-amber text-sm tracking-widest">★★★★★</div>
                <p className="text-sm text-text-secondary leading-relaxed flex-1 italic">
                  <span className="text-primary text-xl not-italic">&ldquo;</span>
                  {r.text}
                </p>
                <div className="flex items-center gap-3 mt-auto">
                  <div
                    className={`w-[42px] h-[42px] rounded-full flex items-center justify-center font-code text-sm font-bold shrink-0 ${r.avatarBg} ${r.avatarColor}`}
                  >
                    {r.initials}
                  </div>
                  <div>
                    <div className="text-sm font-semibold">{r.name}</div>
                    <div className="text-xs text-text-muted">{r.role}</div>
                    <div className="text-xs text-primary-light font-medium">{r.job}</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
