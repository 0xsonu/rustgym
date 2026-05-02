const footerColumns = [
  {
    title: 'Self-Study',
    links: ['Sign Up', 'Rust Course', 'Quest Map', 'Help Center', 'Pricing'],
  },
  {
    title: 'Community',
    links: ['Articles', 'Success Stories', 'Forum', 'Chat', 'Affiliate Program'],
  },
  {
    title: 'Mentorship',
    links: ['Rust Fundamentals', 'Systems Programming', 'WebAssembly Track'],
  },
  {
    title: 'Company',
    links: ['About Us', 'Reviews', 'Press Room', 'FAQ', 'RustGym EDU'],
  },
];

export default function Footer() {
  return (
    <footer className="bg-dark-900 border-t border-border pt-16 pb-8 px-5 md:px-10">
      <div className="max-w-[1200px] mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-[1.4fr_1fr_1fr_1fr_1fr] gap-10 mb-12">
          <div>
            <div className="font-display text-xl font-extrabold tracking-wide mb-3">
              🦀 Rust<span className="text-primary">Gym</span>
            </div>
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
                  <li key={link}>
                    <a
                      href="#"
                      className="text-[13px] text-text-secondary transition-colors hover:text-primary-light"
                    >
                      {link}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
        <div className="border-t border-border pt-7 flex flex-col md:flex-row justify-between items-center gap-4">
          <p className="text-xs text-text-muted">
            © 2026 RustGym. All rights reserved. ·{' '}
            <a href="#" className="text-text-muted hover:text-text-secondary">
              Privacy Policy
            </a>{' '}
            ·{' '}
            <a href="#" className="text-text-muted hover:text-text-secondary">
              Terms of Use
            </a>
          </p>
          <div className="font-code text-xs text-text-muted">
            <span className="text-primary">fn</span> main() {'{'} learn_rust(); {'}'}
          </div>
        </div>
      </div>
    </footer>
  );
}
