import { Link, useLocation, Navigate } from 'react-router-dom';
import { LayoutDashboard, Code2, Map, Users, FileText, Shield } from 'lucide-react';
import { useAuthStore } from '@/stores/authStore';

const navItems = [
  { label: 'Dashboard', href: '/admin', icon: LayoutDashboard },
  { label: 'Challenges', href: '/admin/challenges', icon: Code2 },
  { label: 'Quests', href: '/admin/quests', icon: Map },
  { label: 'Users', href: '/admin/users', icon: Users },
  { label: 'Submissions', href: '/admin/submissions', icon: FileText },
];

export default function AdminLayout({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, user } = useAuthStore();
  const location = useLocation();

  if (!isAuthenticated || user?.role !== 'admin') {
    return <Navigate to="/" replace />;
  }

  return (
    <div className="min-h-screen bg-dark-950 flex">
      {/* Sidebar */}
      <aside className="w-64 bg-dark-900 border-r border-border flex flex-col shrink-0">
        <div className="p-5 border-b border-border">
          <Link to="/admin" className="flex items-center gap-2.5">
            <Shield className="w-5 h-5 text-primary" />
            <span className="font-display text-lg font-bold text-text-primary">Admin Panel</span>
          </Link>
        </div>
        <nav className="flex-1 p-3 space-y-1">
          {navItems.map((item) => {
            const isActive =
              location.pathname === item.href ||
              (item.href !== '/admin' && location.pathname.startsWith(item.href));
            const Icon = item.icon;
            return (
              <Link
                key={item.href}
                to={item.href}
                className={`flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors ${
                  isActive
                    ? 'bg-primary/10 text-primary border border-primary/20'
                    : 'text-text-secondary hover:text-text-primary hover:bg-dark-700'
                }`}
              >
                <Icon className="w-4 h-4" />
                {item.label}
              </Link>
            );
          })}
        </nav>
        <div className="p-4 border-t border-border">
          <Link
            to="/dashboard"
            className="text-xs text-text-muted hover:text-text-primary transition-colors"
          >
            ← Back to App
          </Link>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto">
        <div className="p-8">{children}</div>
      </main>
    </div>
  );
}
