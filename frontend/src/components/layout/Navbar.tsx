import { useState } from 'react';
import { ChevronDown, Menu, X } from 'lucide-react';

const learningLinks = [
  { label: 'Courses', href: '#' },
  { label: 'Tasks', href: '#' },
  { label: 'Quizzes', href: '#' },
  { label: 'Projects', href: '#' },
  { label: 'Help', href: '#' },
  { label: 'Schedule', href: '#' },
];

const communityLinks = [
  { label: 'Forum', href: '#' },
  { label: 'Chat', href: '#' },
  { label: 'Articles', href: '#' },
  { label: 'Success Stories', href: '#' },
  { label: 'Activity', href: '#' },
  { label: 'Reviews', href: '#' },
];

function NavDropdown({
  label,
  links,
}: {
  label: string;
  links: { label: string; href: string }[];
}) {
  return (
    <div className="relative group">
      <button className="flex items-center gap-1 text-[13px] font-medium text-text-secondary px-3 py-1.5 rounded-md transition-colors hover:text-text-primary hover:bg-dark-700">
        {label}
        <ChevronDown className="w-3 h-3 opacity-60" />
      </button>
      <div className="hidden group-hover:block absolute top-full left-0 mt-2 bg-dark-800 border border-border rounded-[10px] p-2 min-w-[180px] z-[200] shadow-[0_20px_60px_rgba(0,0,0,0.5)]">
        {links.map((link) => (
          <a
            key={link.label}
            href={link.href}
            className="block px-3 py-2 text-[13px] text-text-secondary rounded-md transition-all hover:text-text-primary hover:bg-dark-700"
          >
            {link.label}
          </a>
        ))}
      </div>
    </div>
  );
}

export default function Navbar() {
  const [mobileOpen, setMobileOpen] = useState(false);

  return (
    <nav className="fixed top-0 left-0 right-0 z-[100] bg-dark-950/[0.92] backdrop-blur-[12px] border-b border-border h-[62px] px-5 md:px-10 flex items-center gap-8">
      <a
        href="#"
        className="flex items-center gap-2.5 font-display text-[22px] font-extrabold tracking-wide text-text-primary shrink-0"
      >
        <span className="text-2xl">🦀</span>
        <span>
          Rust<span className="text-primary">Gym</span>
        </span>
      </a>

      <div className="hidden md:flex items-center gap-1 flex-1">
        <NavDropdown label="Learning" links={learningLinks} />
        <NavDropdown label="Community" links={communityLinks} />
        <a
          href="#"
          className="text-[13px] font-medium text-text-secondary px-3 py-1.5 rounded-md transition-colors hover:text-text-primary hover:bg-dark-700"
        >
          Pricing
        </a>
      </div>

      <div className="hidden md:block ml-auto">
        <a
          href="#"
          className="bg-primary text-white px-5 py-2 rounded-lg text-[13px] font-semibold tracking-wide transition-all hover:bg-primary-light hover:-translate-y-px"
        >
          Start Free →
        </a>
      </div>

      {/* Mobile toggle */}
      <button
        className="md:hidden ml-auto text-text-primary"
        onClick={() => setMobileOpen(!mobileOpen)}
        aria-label="Toggle menu"
      >
        {mobileOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
      </button>

      {/* Mobile menu */}
      {mobileOpen && (
        <div className="md:hidden absolute top-[62px] left-0 right-0 bg-dark-900 border-b border-border p-4 flex flex-col gap-3">
          <a href="#" className="text-sm text-text-secondary hover:text-text-primary py-2">
            Learning
          </a>
          <a href="#" className="text-sm text-text-secondary hover:text-text-primary py-2">
            Community
          </a>
          <a href="#" className="text-sm text-text-secondary hover:text-text-primary py-2">
            Pricing
          </a>
          <a
            href="#"
            className="bg-primary text-white px-5 py-2 rounded-lg text-[13px] font-semibold text-center mt-2"
          >
            Start Free →
          </a>
        </div>
      )}
    </nav>
  );
}
