import { useState, useRef, useEffect, useCallback } from 'react';
import { Link, useNavigate, useLocation } from 'react-router-dom';
import {
  Bug,
  ChevronDown,
  Menu,
  X,
  User,
  LayoutDashboard,
  LogOut,
  Trophy,
  BookOpen,
} from 'lucide-react';
import { AnimatePresence, motion } from 'framer-motion';
import { useAuthStore } from '@/stores/authStore';
import { duration, easing } from '@/lib/motion';

const learningLinks = [
  { label: 'Quest Map', to: '/quests' },
  { label: 'Dashboard', to: '/dashboard' },
  { label: 'Leaderboard', to: '/leaderboard' },
];

const communityLinks = [
  { label: 'Forum', to: '/forum' },
  { label: 'Articles', to: '/articles' },
];

function NavDropdown({
  label,
  links,
  isActive,
  onNavigate,
}: {
  label: string;
  links: { label: string; to: string }[];
  isActive?: boolean;
  onNavigate?: () => void;
}) {
  const [open, setOpen] = useState(false);
  const [focusedIndex, setFocusedIndex] = useState(-1);
  const containerRef = useRef<HTMLDivElement>(null);
  const itemRefs = useRef<(HTMLAnchorElement | null)[]>([]);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setOpen(false);
        setFocusedIndex(-1);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      switch (e.key) {
        case 'Enter':
        case ' ':
          e.preventDefault();
          setOpen((prev) => !prev);
          if (!open) setFocusedIndex(0);
          break;
        case 'Escape':
          e.preventDefault();
          setOpen(false);
          setFocusedIndex(-1);
          break;
        case 'ArrowDown':
          e.preventDefault();
          if (!open) {
            setOpen(true);
            setFocusedIndex(0);
          } else {
            setFocusedIndex((prev) => Math.min(prev + 1, links.length - 1));
          }
          break;
        case 'ArrowUp':
          e.preventDefault();
          if (open) {
            setFocusedIndex((prev) => Math.max(prev - 1, 0));
          }
          break;
      }
    },
    [open, links.length],
  );

  useEffect(() => {
    if (focusedIndex >= 0 && itemRefs.current[focusedIndex]) {
      itemRefs.current[focusedIndex]?.focus();
    }
  }, [focusedIndex]);

  return (
    <div className="relative" ref={containerRef}>
      <button
        className={`flex items-center gap-1 text-[13px] font-medium px-3 py-1.5 rounded-md transition-colors duration-200 hover:text-text-primary hover:bg-surface-elevated ${
          isActive ? 'text-text-primary' : 'text-text-secondary'
        }`}
        onClick={() => setOpen((prev) => !prev)}
        onKeyDown={handleKeyDown}
        aria-expanded={open}
        aria-haspopup="true"
      >
        {label}
        <ChevronDown
          className={`w-3 h-3 opacity-60 transition-transform duration-200 ${open ? 'rotate-180' : ''}`}
        />
        {isActive && (
          <span className="absolute bottom-0 left-3 right-3 h-0.5 bg-primary rounded-full" />
        )}
      </button>
      {open && (
        <div
          className="absolute top-full left-0 mt-2 bg-surface-elevated border border-border rounded-lg p-2 min-w-[180px] z-[200] shadow-lg"
          role="menu"
        >
          {links.map((link, index) => (
            <Link
              key={link.label}
              to={link.to}
              ref={(el) => {
                itemRefs.current[index] = el;
              }}
              role="menuitem"
              tabIndex={focusedIndex === index ? 0 : -1}
              onClick={() => {
                setOpen(false);
                setFocusedIndex(-1);
                onNavigate?.();
              }}
              onKeyDown={(e) => {
                if (e.key === 'Escape') {
                  e.preventDefault();
                  setOpen(false);
                  setFocusedIndex(-1);
                } else if (e.key === 'ArrowDown') {
                  e.preventDefault();
                  setFocusedIndex(Math.min(index + 1, links.length - 1));
                } else if (e.key === 'ArrowUp') {
                  e.preventDefault();
                  setFocusedIndex(Math.max(index - 1, 0));
                }
              }}
              className="block px-3 py-2 text-[13px] text-text-secondary rounded-md transition-colors duration-200 hover:text-text-primary hover:bg-surface-overlay focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-1 focus-visible:ring-offset-surface-elevated"
            >
              {link.label}
            </Link>
          ))}
        </div>
      )}
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
  const [dropdownFocusIndex, setDropdownFocusIndex] = useState(-1);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const dropdownItemRefs = useRef<(HTMLAnchorElement | HTMLButtonElement | null)[]>([]);
  const navigate = useNavigate();
  const location = useLocation();

  const { isAuthenticated, user, logout } = useAuthStore();

  // Check if a route is active
  const isRouteActive = (path: string) => location.pathname === path;
  const isGroupActive = (links: { to: string }[]) =>
    links.some((link) => location.pathname.startsWith(link.to));

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setDropdownOpen(false);
        setDropdownFocusIndex(-1);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Close mobile menu on route change
  const prevPathname = useRef(location.pathname);
  useEffect(() => {
    if (prevPathname.current !== location.pathname) {
      prevPathname.current = location.pathname;
      setMobileOpen(false);
    }
  }, [location.pathname]);

  // Lock body scroll when mobile menu is open
  useEffect(() => {
    if (mobileOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [mobileOpen]);

  const handleUserDropdownKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      const items = dropdownItemRefs.current.filter(Boolean);
      switch (e.key) {
        case 'Enter':
        case ' ':
          e.preventDefault();
          setDropdownOpen((prev) => !prev);
          if (!dropdownOpen) setDropdownFocusIndex(0);
          break;
        case 'Escape':
          e.preventDefault();
          setDropdownOpen(false);
          setDropdownFocusIndex(-1);
          break;
        case 'ArrowDown':
          e.preventDefault();
          if (!dropdownOpen) {
            setDropdownOpen(true);
            setDropdownFocusIndex(0);
          } else {
            setDropdownFocusIndex((prev) => Math.min(prev + 1, items.length - 1));
          }
          break;
        case 'ArrowUp':
          e.preventDefault();
          if (dropdownOpen) {
            setDropdownFocusIndex((prev) => Math.max(prev - 1, 0));
          }
          break;
      }
    },
    [dropdownOpen],
  );

  useEffect(() => {
    if (dropdownFocusIndex >= 0 && dropdownItemRefs.current[dropdownFocusIndex]) {
      dropdownItemRefs.current[dropdownFocusIndex]?.focus();
    }
  }, [dropdownFocusIndex]);

  // Build user dropdown items
  const userDropdownItems = [
    { label: 'Profile', to: `/profile/${user?.username ?? ''}`, icon: User },
    { label: 'Dashboard', to: '/dashboard', icon: LayoutDashboard },
    { label: 'Quests', to: '/quests', icon: BookOpen },
    { label: 'Leaderboard', to: '/leaderboard', icon: Trophy },
    ...(user?.role === 'admin'
      ? [{ label: 'Admin Panel', to: '/admin', icon: LayoutDashboard }]
      : []),
  ];

  return (
    <nav className="fixed top-0 left-0 right-0 z-[100] bg-surface-base/[0.92] backdrop-blur-md border-b border-border h-[62px] px-5 md:px-10 flex items-center gap-8">
      {/* Logo */}
      <Link
        to="/"
        className="flex items-center gap-2.5 font-display text-[22px] font-extrabold tracking-wide text-text-primary shrink-0"
      >
        <Bug className="w-6 h-6 text-primary" aria-hidden="true" />
        <span>
          Rust<span className="text-primary">Gym</span>
        </span>
      </Link>

      {/* Desktop navigation */}
      <div className="hidden md:flex items-center gap-1 flex-1">
        <NavDropdown
          label="Learning"
          links={learningLinks}
          isActive={isGroupActive(learningLinks)}
        />
        <NavDropdown
          label="Community"
          links={communityLinks}
          isActive={isGroupActive(communityLinks)}
        />
        <Link
          to="/leaderboard"
          className={`relative text-[13px] font-medium px-3 py-1.5 rounded-md transition-colors duration-200 hover:text-text-primary hover:bg-surface-elevated ${
            isRouteActive('/leaderboard') ? 'text-text-primary' : 'text-text-secondary'
          }`}
        >
          Leaderboard
          {isRouteActive('/leaderboard') && (
            <span className="absolute bottom-0 left-3 right-3 h-0.5 bg-primary rounded-full" />
          )}
        </Link>
      </div>

      {/* Desktop auth section */}
      <div className="hidden md:flex items-center gap-3 ml-auto">
        {!isAuthenticated ? (
          <>
            <Link
              to="/login"
              className="text-[13px] font-medium text-text-secondary px-4 py-2 rounded-lg transition-colors duration-200 hover:text-text-primary hover:bg-surface-elevated"
            >
              Login
            </Link>
            <Link
              to="/register"
              className="bg-primary text-white px-5 py-2 rounded-lg text-[13px] font-semibold tracking-wide transition-all duration-200 hover:bg-primary-600 hover:-translate-y-px active:scale-[0.98]"
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
              onKeyDown={handleUserDropdownKeyDown}
              className="flex items-center gap-2 rounded-lg px-2 py-1.5 transition-colors duration-200 hover:bg-surface-elevated focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-surface-base"
              aria-expanded={dropdownOpen}
              aria-haspopup="true"
              aria-label="User menu"
            >
              <UserAvatar username={user?.username ?? ''} />
              <ChevronDown
                className={`w-3 h-3 text-text-muted transition-transform duration-200 ${dropdownOpen ? 'rotate-180' : ''}`}
              />
            </button>

            {dropdownOpen && (
              <div
                className="absolute top-full right-0 mt-2 mr-5 bg-surface-elevated border border-border rounded-lg p-2 min-w-[180px] z-[200] shadow-lg"
                role="menu"
              >
                <div className="px-3 py-2 border-b border-border mb-1">
                  <p className="text-sm font-medium text-text-primary">{user?.username}</p>
                  <p className="text-xs text-text-muted">{user?.email}</p>
                </div>
                {userDropdownItems.map((item, index) => (
                  <Link
                    key={item.label}
                    to={item.to}
                    ref={(el) => {
                      dropdownItemRefs.current[index] = el;
                    }}
                    role="menuitem"
                    tabIndex={dropdownFocusIndex === index ? 0 : -1}
                    onClick={() => {
                      setDropdownOpen(false);
                      setDropdownFocusIndex(-1);
                    }}
                    onKeyDown={(e) => {
                      if (e.key === 'Escape') {
                        e.preventDefault();
                        setDropdownOpen(false);
                        setDropdownFocusIndex(-1);
                      }
                    }}
                    className="flex items-center gap-2 px-3 py-2 text-[13px] text-text-secondary rounded-md transition-colors duration-200 hover:text-text-primary hover:bg-surface-overlay focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
                  >
                    <item.icon className="w-4 h-4" aria-hidden="true" />
                    {item.label}
                  </Link>
                ))}
                <button
                  ref={(el) => {
                    dropdownItemRefs.current[userDropdownItems.length] = el;
                  }}
                  role="menuitem"
                  tabIndex={dropdownFocusIndex === userDropdownItems.length ? 0 : -1}
                  onClick={() => {
                    setDropdownOpen(false);
                    setDropdownFocusIndex(-1);
                    void logout();
                    navigate('/');
                  }}
                  onKeyDown={(e) => {
                    if (e.key === 'Escape') {
                      e.preventDefault();
                      setDropdownOpen(false);
                      setDropdownFocusIndex(-1);
                    }
                  }}
                  className="flex items-center gap-2 w-full px-3 py-2 text-[13px] text-error rounded-md transition-colors duration-200 hover:bg-surface-overlay border-t border-border mt-1 pt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
                >
                  <LogOut className="w-4 h-4" aria-hidden="true" />
                  Logout
                </button>
              </div>
            )}
          </div>
        )}
      </div>

      {/* Mobile toggle */}
      <button
        className="md:hidden ml-auto text-text-primary p-1 rounded-md transition-colors duration-200 hover:bg-surface-elevated focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
        onClick={() => setMobileOpen(!mobileOpen)}
        aria-label={mobileOpen ? 'Close menu' : 'Open menu'}
        aria-expanded={mobileOpen}
      >
        {mobileOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
      </button>

      {/* Mobile menu — full-screen overlay with slide-in */}
      <AnimatePresence>
        {mobileOpen && (
          <motion.div
            className="md:hidden fixed inset-0 top-[62px] z-[99] bg-surface-base/95 backdrop-blur-md flex flex-col p-6 overflow-y-auto"
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ duration: duration.normal, ease: easing.easeOut }}
          >
            <div className="flex flex-col gap-1">
              <p className="text-xs font-semibold text-text-muted uppercase tracking-wider mb-2 px-3">
                Learning
              </p>
              {learningLinks.map((link) => (
                <Link
                  key={link.label}
                  to={link.to}
                  className={`text-sm px-3 py-2.5 rounded-md transition-colors duration-200 ${
                    isRouteActive(link.to)
                      ? 'text-text-primary bg-surface-elevated border-l-2 border-primary'
                      : 'text-text-secondary hover:text-text-primary hover:bg-surface-elevated'
                  }`}
                >
                  {link.label}
                </Link>
              ))}
            </div>

            <div className="flex flex-col gap-1 mt-6">
              <p className="text-xs font-semibold text-text-muted uppercase tracking-wider mb-2 px-3">
                Community
              </p>
              {communityLinks.map((link) => (
                <Link
                  key={link.label}
                  to={link.to}
                  className={`text-sm px-3 py-2.5 rounded-md transition-colors duration-200 ${
                    isRouteActive(link.to)
                      ? 'text-text-primary bg-surface-elevated border-l-2 border-primary'
                      : 'text-text-secondary hover:text-text-primary hover:bg-surface-elevated'
                  }`}
                >
                  {link.label}
                </Link>
              ))}
            </div>

            <div className="border-t border-border mt-6 pt-6">
              {!isAuthenticated ? (
                <div className="flex flex-col gap-3">
                  <Link
                    to="/login"
                    className="text-sm text-text-secondary hover:text-text-primary px-3 py-2.5 rounded-md transition-colors duration-200 hover:bg-surface-elevated"
                  >
                    Login
                  </Link>
                  <Link
                    to="/register"
                    className="bg-primary text-white px-5 py-2.5 rounded-lg text-[13px] font-semibold text-center transition-all duration-200 hover:bg-primary-600 active:scale-[0.98]"
                  >
                    Start Free →
                  </Link>
                </div>
              ) : (
                <div className="flex flex-col gap-1">
                  <div className="flex items-center gap-3 px-3 py-2 mb-3">
                    <UserAvatar username={user?.username ?? ''} />
                    <div>
                      <p className="text-sm font-medium text-text-primary">{user?.username}</p>
                      <p className="text-xs text-text-muted">
                        {user?.xp ?? 0} XP · Level {user?.level ?? 1}
                      </p>
                    </div>
                  </div>
                  <Link
                    to="/dashboard"
                    className="text-sm text-text-secondary hover:text-text-primary px-3 py-2.5 rounded-md transition-colors duration-200 hover:bg-surface-elevated"
                  >
                    Dashboard
                  </Link>
                  <Link
                    to={`/profile/${user?.username ?? ''}`}
                    className="text-sm text-text-secondary hover:text-text-primary px-3 py-2.5 rounded-md transition-colors duration-200 hover:bg-surface-elevated"
                  >
                    Profile
                  </Link>
                  <button
                    onClick={() => {
                      void logout();
                      navigate('/');
                    }}
                    className="text-sm text-error hover:text-error/80 px-3 py-2.5 rounded-md text-left transition-colors duration-200 hover:bg-surface-elevated mt-2 border-t border-border pt-4"
                  >
                    Logout
                  </button>
                </div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </nav>
  );
}
