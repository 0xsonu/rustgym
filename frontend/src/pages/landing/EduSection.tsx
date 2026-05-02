import { FadeIn } from '../Landing';

const eduStats = [
  { num: '1,200+', label: 'Students in EDU Program' },
  { num: '48', label: 'Learning Groups' },
  { num: '22', label: 'Countries' },
];

export default function EduSection() {
  return (
    <section className="py-24 px-5 md:px-10 bg-dark-900">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          For Educators
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          Bring Rust to your <span className="text-primary">classroom</span>
        </h2>

        <FadeIn>
          <div className="bg-dark-card border border-border rounded-3xl p-10 md:p-[60px] grid grid-cols-1 md:grid-cols-[1fr_auto] items-center gap-10 md:gap-[60px] mt-12">
            <div>
              <p className="text-[17px] text-text-secondary max-w-[540px] leading-relaxed mb-7">
                RustGym offers a 6-month FREE trial for educators and their students. Set up
                learning groups, track progress, and integrate Rust into your curriculum with zero
                friction.
              </p>
              <div className="flex gap-12 flex-wrap">
                {eduStats.map((s) => (
                  <div key={s.label}>
                    <div className="font-display text-4xl font-extrabold text-primary">{s.num}</div>
                    <div className="text-[13px] text-text-muted mt-0.5">{s.label}</div>
                  </div>
                ))}
              </div>
            </div>
            <div>
              <a
                href="#"
                className="inline-block bg-primary border-2 border-primary text-white px-8 py-3.5 rounded-[10px] text-[15px] font-semibold whitespace-nowrap transition-all hover:bg-primary-light hover:border-primary-light"
              >
                Apply for EDU Access →
              </a>
            </div>
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
