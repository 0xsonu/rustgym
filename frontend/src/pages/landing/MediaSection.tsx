import { FadeIn } from '../Landing';

const mediaOutlets = [
  'The Register',
  'HackerNews',
  'InfoQ',
  'freeCodeCamp',
  'This Week in Rust',
  'DEV.to',
  'LogRocket Blog',
  'TechCrunch',
];

export default function MediaSection() {
  return (
    <section className="py-24 px-5 md:px-10">
      <div className="max-w-[1200px] mx-auto text-center">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          In the press
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          RustGym in <span className="text-primary">Media</span>
        </h2>

        <FadeIn>
          <div className="flex flex-wrap justify-center gap-3 mt-12">
            {mediaOutlets.map((name) => (
              <div
                key={name}
                className="bg-dark-card border border-border rounded-[10px] px-6 py-3 text-sm font-semibold text-text-secondary transition-all hover:text-text-primary hover:border-border-light"
              >
                {name}
              </div>
            ))}
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
