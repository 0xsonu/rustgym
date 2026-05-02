import { FadeIn } from '../Landing';

const prospects = [
  {
    num: '$140K',
    title: 'Average Rust Developer Salary (US)',
    desc: 'Rust engineers command some of the highest salaries in the industry — significantly above language averages. Companies pay for safety and performance expertise.',
  },
  {
    num: '9×',
    title: 'Most Loved Language in a Row',
    desc: 'Rust has topped the Stack Overflow Developer Survey\'s "most admired" list for 9 consecutive years. Developers who learn it, love it and keep using it.',
  },
  {
    num: '∞',
    title: 'Used Everywhere That Matters',
    desc: 'Linux kernel, Windows, Android, Firefox, Cloudflare, AWS, Discord, Dropbox, Meta — Rust is at the foundation of modern infrastructure. Learn it once, work anywhere.',
  },
];

export default function ProspectsSection() {
  return (
    <section className="py-24 px-5 md:px-10">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          Why Rust?
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          Rust is a <span className="text-primary">career-defining</span> choice
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed">
          The most loved language for 9 consecutive years. Growing demand, premium salaries, and
          used at the core of the internet.
        </p>

        <FadeIn>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-16">
            {prospects.map((p) => (
              <div
                key={p.title}
                className="bg-dark-card border border-border rounded-2xl p-7 transition-all hover:border-border-light hover:-translate-y-[3px]"
              >
                <div className="font-display text-5xl font-extrabold text-primary leading-none mb-2">
                  {p.num}
                </div>
                <h4 className="text-base font-semibold mb-2">{p.title}</h4>
                <p className="text-[13px] text-text-secondary leading-relaxed">{p.desc}</p>
              </div>
            ))}
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
