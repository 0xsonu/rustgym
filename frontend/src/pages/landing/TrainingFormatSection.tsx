import { Link } from 'react-router-dom';
import { ArrowRight } from 'lucide-react';
import { Button } from '@/components/ui';
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
    <section className="py-24 px-5 md:px-10 bg-surface-elevated">
      <div className="max-w-[1200px] mx-auto">
        <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3">
          Training Format
        </div>
        <h2 className="font-display text-[clamp(34px,4vw,52px)] font-extrabold leading-tight mb-4">
          Choose how you <span className="text-primary">train</span>
        </h2>
        <p className="text-[17px] text-text-secondary max-w-[580px] leading-relaxed font-body">
          Go at your own pace or level up faster with live mentorship. Both paths lead to real Rust
          mastery.
        </p>

        <FadeIn>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-16">
            {/* Self-Paced */}
            <div className="bg-surface-base border border-border rounded-[20px] p-10 relative overflow-hidden">
              <h3 className="font-display text-[26px] font-extrabold mb-2">Self-Paced</h3>
              <p className="text-sm text-text-secondary mb-7 leading-relaxed font-body">
                The complete Rust curriculum — quests, tasks, mini-projects — on your schedule.
              </p>
              <ul className="flex flex-col gap-3 mb-8">
                {selfPacedFeatures.map((item) => (
                  <li
                    key={item}
                    className="flex gap-2.5 text-sm text-text-secondary items-start font-body"
                  >
                    <ArrowRight
                      className="w-4 h-4 text-primary shrink-0 mt-0.5"
                      aria-hidden="true"
                    />
                    {item}
                  </li>
                ))}
              </ul>
              <Button asChild variant="secondary" size="lg">
                <Link to="/register">Start for Free</Link>
              </Button>
            </div>

            {/* Live Mentorship */}
            <div className="bg-surface-base border border-primary rounded-[20px] p-10 relative overflow-hidden">
              <div className="absolute top-5 right-5 bg-primary text-white text-[11px] font-bold px-2.5 py-1 rounded-xl tracking-wide uppercase">
                Popular
              </div>
              <h3 className="font-display text-[26px] font-extrabold mb-2">Live Mentorship</h3>
              <p className="text-sm text-text-secondary mb-7 leading-relaxed font-body">
                Guided cohorts with senior Rust engineers — twice-weekly live sessions, real
                projects, and job prep.
              </p>
              <ul className="flex flex-col gap-3 mb-8">
                {mentorshipFeatures.map((item) => (
                  <li
                    key={item}
                    className="flex gap-2.5 text-sm text-text-secondary items-start font-body"
                  >
                    <ArrowRight
                      className="w-4 h-4 text-primary shrink-0 mt-0.5"
                      aria-hidden="true"
                    />
                    {item}
                  </li>
                ))}
              </ul>
              <Button asChild size="lg">
                <Link to="/register">Learn More</Link>
              </Button>
            </div>
          </div>
        </FadeIn>
      </div>
    </section>
  );
}
