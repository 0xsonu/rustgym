import { useState, useRef, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { ChevronDown, Menu, X, User, LayoutDashboard, LogOut } from 'lucide-react';
import { useAuthStore } from '@/stores/authStore';

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

function UserAvatar({ username }: { username: string }) {
  const initials = username.slice(0, 2).toUpperCase();
  return (
    <div className="w-8 h-8 rounded-full bg-primary/20 border border-primary/40 flex items-center justify-center">
      <span className="text-xs font-bold text-primary">{initials}</span>
    </div>
  );
}

function LevelBadge({ level }: { level: number }) {
  return (
    <span className="inline-flex items-center justify-center w-6 h-6 rounded-full bg-amber/20 border border-amber/40 text-[10px] font-bold text-amber">
      {level}
    </span>
  );
}

export default function Navbar() {
  const [mobileOpen, setMobileOpen] = useState(false);
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  const { isAuthenticated, user, logout } = useAuthStore();

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setDropdownOpen(false);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  return (
    <nav className="fixed top-0 left-0 right-0 z-[100] bg-dark-950/[0.92] backdrop-blur-[12px] border-b border-border h-[62px] px-5 md:px-10 flex items-center gap-8">
      <Link
        to="/"
        className="flex items-center gap-2.5 font-display text-[22px] font-extrabold tracking-wide text-text-primary shrink-0"
      >
        <span className="text-2xl">🦀</span>
        <span>
          Rust<span className="text-primary">Gym</span>
        </span>
      </Link>

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

      <div className="hidden md:flex items-center gap-3 ml-auto">
        {!isAuthenticated ? (
          <>
            <Link
              to="/login"
              className="text-[13px] font-medium text-text-secondary px-4 py-2 rounded-lg transition-colors hover:text-text-primary hover:bg-dark-700"
            >
              Login
            </Link>
            <Link
              to="/register"
              className="bg-primary text-white px-5 py-2 rounded-lg text-[13px] font-semibold tracking-wide transition-all hover:bg-primary-light hover:-translate-y-px"
            >
              Start Free →
            </Link>
          </>
        ) : (
          <div className="flex items-center gap-3" ref={dropdownRef}>
            <div className="flex items-center gap-2 text-sm text-text-secondary">
              <span className="font-code text-xs text-amber">{user?.xp ?? 0} XP</span>
              <LevelBadge level={user?.level ?? 1} />
            </div>

            <button
              onClick={() => setDropdownOpen(!dropdownOpen)}
              className="flex items-center gap-2 rounded-lg px-2 py-1.5 transition-colors hover:bg-dark-700"
            >
              <UserAvatar username={user?.username ?? ''} />
              <ChevronDown className="w-3 h-3 text-text-muted" />
            </button>

            {dropdownOpen && (
              <div className="absolute top-full right-0 mt-2 mr-5 bg-dark-800 border border-border rounded-[10px] p-2 min-w-[180px] z-[200] shadow-[0_20px_60px_rgba(0,0,0,0.5)]">
                <div className="px-3 py-2 border-b border-border mb-1">
                  <p className="text-sm font-medium text-text-primary">{user?.username}</p>
                  <p className="text-xs text-text-muted">{user?.email}</p>
                </div>
                <a
                  href="#"
                  className="flex items-center gap-2 px-3 py-2 text-[13px] text-text-secondary rounded-md transition-all hover:text-text-primary hover:bg-dark-700"
                >
                  <User className="w-4 h-4" />
                  Profile
                </a>
                <a
                  href="#"
                  className="flex items-center gap-2 px-3 py-2 text-[13px] text-text-secondary rounded-md transition-all hover:text-text-primary hover:bg-dark-700"
                >
                  <LayoutDashboard className="w-4 h-4" />
                  Dashboard
                </a>
                <button
                  onClick={() => {
                    setDropdownOpen(false);
                    void logout();
                  }}
                  className="flex items-center gap-2 w-full px-3 py-2 text-[13px] text-red-400 rounded-md transition-all hover:bg-dark-700"
                >
                  <LogOut className="w-4 h-4" />
                  Logout
                </button>
              </div>
            )}
          </div>
        )}
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
          {!isAuthenticated ? (
            <>
              <Link
                to="/login"
                className="text-sm text-text-secondary hover:text-text-primary py-2"
              >
                Login
              </Link>
              <Link
                to="/register"
                className="bg-primary text-white px-5 py-2 rounded-lg text-[13px] font-semibold text-center mt-2"
              >
                Start Free →
              </Link>
            </>
          ) : (
            <button
              onClick={() => {
                setMobileOpen(false);
                void logout();
              }}
              className="text-sm text-red-400 hover:text-red-300 py-2 text-left"
            >
              Logout
            </button>
          )}
        </div>
      )}
    </nav>
  );
}
