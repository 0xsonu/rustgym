import { Link } from 'react-router-dom';

const footerColumns = [
  {
    title: 'Self-Study',
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
    links: [
      { label: 'Rust Fundamentals', to: '/quests' },
      { label: 'Systems Programming', to: '/quests' },
      { label: 'WebAssembly Track', to: '/quests' },
    ],
  },
  {
    title: 'Company',
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
    <footer className="bg-dark-900 border-t border-border pt-16 pb-8 px-5 md:px-10">
      <div className="max-w-[1200px] mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-[1.4fr_1fr_1fr_1fr_1fr] gap-10 mb-12">
          <div>
            <Link
              to="/"
              className="font-display text-xl font-extrabold tracking-wide mb-3 inline-block"
            >
              🦀 Rust<span className="text-primary">Gym</span>
            </Link>
            <p className="text-[13px] text-text-muted leading-relaxed max-w-[220px]">
              An online course for learning Rust programming from scratch — 1,400+ tasks, gamified
              quests, and a community of passionate systems programmers.
            </p>
          </div>
          {footerColumns.map((col) => (
            <div key={col.title}>
              <h4 className="text-xs font-bold tracking-[1.5px] uppercase text-text-muted mb-4">
                {col.title}
              </h4>
              <ul className="flex flex-col gap-2.5">
                {col.links.map((link) => (
                  <li key={link.label}>
                    {link.to.startsWith('#') ? (
                      <span className="text-[13px] text-text-secondary cursor-default">
                        {link.label}
                      </span>
                    ) : (
                      <Link
                        to={link.to}
                        className="text-[13px] text-text-secondary transition-colors hover:text-primary-light"
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
        <div className="border-t border-border pt-7 flex flex-col md:flex-row justify-between items-center gap-4">
          <p className="text-xs text-text-muted">© 2026 RustGym. All rights reserved.</p>
          <div className="font-code text-xs text-text-muted">
            <span className="text-primary">fn</span> main() {'{'} learn_rust(); {'}'}
          </div>
        </div>
      </div>
    </footer>
  );
}
