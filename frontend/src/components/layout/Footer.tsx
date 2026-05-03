import { Link } from 'react-router-dom';
import { Bug, BookOpen, Users, GraduationCap, Building2 } from 'lucide-react';

const footerColumns = [
  {
    title: 'Self-Study',
    icon: BookOpen,
    links: [
      { label: 'Sign Up', to: '/register' },
      { label: 'Rust Course', to: '/quests' },
      { label: 'Quest Map', to: '/quests' },
      { label: 'Help Center', to: '#' },
      { label: 'Pricing', to: '#' },
    ],
  },
  {
    title: 'Community',
    icon: Users,
    links: [
      { label: 'Articles', to: '#' },
      { label: 'Success Stories', to: '#' },
      { label: 'Forum', to: '#' },
      { label: 'Chat', to: '#' },
      { label: 'Affiliate Program', to: '#' },
    ],
  },
  {
    title: 'Mentorship',
    icon: GraduationCap,
    links: [
      { label: 'Rust Fundamentals', to: '/quests' },
      { label: 'Systems Programming', to: '/quests' },
      { label: 'WebAssembly Track', to: '/quests' },
    ],
  },
  {
    title: 'Company',
    icon: Building2,
    links: [
      { label: 'About Us', to: '#' },
      { label: 'Reviews', to: '#' },
      { label: 'Press Room', to: '#' },
      { label: 'FAQ', to: '#' },
      { label: 'RustGym EDU', to: '#' },
    ],
  },
];

export default function Footer() {
  return (
    <footer className="bg-surface-base border-t border-border pt-16 pb-8 px-5 md:px-10 font-body">
      <div className="max-w-[1280px] mx-auto">
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-[1.4fr_1fr_1fr_1fr_1fr] gap-10 mb-12">
          {/* Brand column */}
          <div>
            <Link
              to="/"
              className="font-display text-xl font-extrabold tracking-wide mb-3 inline-flex items-center gap-2 text-text-primary"
            >
              <Bug className="w-6 h-6 text-primary" aria-hidden="true" />
              Rust<span className="text-primary">Gym</span>
            </Link>
            <p className="text-sm text-text-muted leading-relaxed max-w-[220px] font-body">
              An online course for learning Rust programming from scratch — 1,400+ tasks, gamified
              quests, and a community of passionate systems programmers.
            </p>
          </div>

          {/* Link columns */}
          {footerColumns.map((col) => (
            <div key={col.title}>
              <h4 className="font-display text-xs font-bold tracking-[1.5px] uppercase text-text-secondary mb-4">
                {col.title}
              </h4>
              <ul className="flex flex-col gap-2.5">
                {col.links.map((link) => (
                  <li key={link.label}>
                    {link.to.startsWith('#') ? (
                      <span className="text-sm text-text-muted cursor-default font-body">
                        {link.label}
                      </span>
                    ) : (
                      <Link
                        to={link.to}
                        className="text-sm text-text-secondary font-body transition-colors duration-200 hover:text-primary"
                      >
                        {link.label}
                      </Link>
                    )}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        {/* Bottom bar */}
        <div className="border-t border-border pt-7 flex flex-col md:flex-row justify-between items-center gap-4">
          <p className="text-xs text-text-muted font-body">© 2026 RustGym. All rights reserved.</p>
          <div className="font-code text-xs text-text-muted">
            <span className="text-primary">fn</span> main() {'{'} learn_rust(); {'}'}
          </div>
        </div>
      </div>
    </footer>
  );
}
