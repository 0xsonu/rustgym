import { FadeIn } from '../Landing';

const selfPacedFeatures = [
  '1,400+ coding tasks with instant verification',
  'In-browser Rust IDE with Cargo integration',
  'Hints, solutions, and community help',
  'Achievements and XP leaderboard',
  'Mobile app — learn on the go',
  'Save progress and set your own schedule',
];

const mentorshipFeatures = [
  '2 live sessions / week with expert mentors',
  'Large final projects (CLI tools, web servers, WASM)',
  'Daily support in private group chat',
  'Code review from experienced Rustaceans',
  'Certificate of completion',
  'Resume review & interview prep assistance',
];

export default function TrainingFormatSection() {
  return (
    <section className="py-24 px-5 md:px-10 bg-dark-900">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          Training Format
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          Choose how you <span className="text-primary">train</span>
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed">
          Go at your own pace or level up faster with live mentorship. Both paths lead to real Rust
          mastery.
        </p>

        <FadeIn>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-16">
            {/* Self-Paced */}
            <div className="bg-dark-card border border-border rounded-[20px] p-10 relative overflow-hidden">
              <h3 className="font-display text-[26px] font-extrabold mb-2">Self-Paced</h3>
              <p className="text-sm text-text-secondary mb-7 leading-relaxed">
                The complete Rust curriculum — quests, tasks, mini-projects — on your schedule.
              </p>
              <ul className="flex flex-col gap-3 mb-8">
                {selfPacedFeatures.map((item) => (
                  <li key={item} className="flex gap-2.5 text-sm text-text-secondary items-start">
                    <span className="text-primary font-code text-[13px] shrink-0 mt-px">→</span>
                    {item}
                  </li>
                ))}
              </ul>
              <a
                href="#"
                className="inline-block px-6 py-3 rounded-[10px] text-sm font-semibold border-2 border-border-light text-text-primary transition-all hover:border-primary hover:text-primary"
              >
                Start for Free
              </a>
            </div>

            {/* Live Mentorship */}
            <div className="bg-dark-card border border-primary rounded-[20px] p-10 relative overflow-hidden">
              <div className="absolute top-5 right-5 bg-primary text-white text-[11px] font-bold px-2.5 py-1 rounded-xl tracking-wide uppercase">
                Popular
              </div>
              <h3 className="font-display text-[26px] font-extrabold mb-2">Live Mentorship</h3>
              <p className="text-sm text-text-secondary mb-7 leading-relaxed">
                Guided cohorts with senior Rust engineers — twice-weekly live sessions, real
                projects, and job prep.
              </p>
              <ul className="flex flex-col gap-3 mb-8">
                {mentorshipFeatures.map((item) => (
                  <li key={item} className="flex gap-2.5 text-sm text-text-secondary items-start">
                    <span className="text-primary font-code text-[13px] shrink-0 mt-px">→</span>
                    {item}
                  </li>
                ))}
              </ul>
              <a
                href="#"
                className="inline-block px-6 py-3 rounded-[10px] text-sm font-semibold bg-primary border-2 border-primary text-white transition-all hover:bg-primary-light hover:border-primary-light"
              >
                Learn More
              </a>
            </div>
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
