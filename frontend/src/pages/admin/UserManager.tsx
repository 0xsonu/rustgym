import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Search, Ban, ChevronLeft, ChevronRight } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import AdminLayout from './AdminLayout';

export default function UserManager() {
  const queryClient = useQueryClient();
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const perPage = 20;

  const { data, isLoading } = useQuery({
    queryKey: ['admin', 'users', { page, search }],
    queryFn: () => adminApi.listUsers({ page, per_page: perPage, search: search || undefined }),
  });

  const updateRoleMutation = useMutation({
    mutationFn: ({ id, role }: { id: string; role: string }) => adminApi.updateUserRole(id, role),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'users'] });
    },
  });

  const banMutation = useMutation({
    mutationFn: (id: string) => adminApi.banUser(id),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['admin', 'users'] });
    },
  });

  const totalPages = data ? Math.ceil(data.total / perPage) : 0;

  return (
    <AdminLayout>
      <div>
        <h1 className="font-display text-2xl font-bold text-text-primary mb-6">User Manager</h1>

        {/* Search */}
        <div className="relative mb-4 max-w-md">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" />
          <input
            type="text"
            value={search}
            onChange={(e) => {
              setSearch(e.target.value);
              setPage(1);
            }}
            placeholder="Search by username or email..."
            className="w-full bg-dark-card border border-border rounded-lg pl-9 pr-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-primary"
          />
        </div>

        {/* Users Table */}
        <div className="bg-dark-card border border-border rounded-xl overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="text-left text-xs text-text-muted border-b border-border bg-dark-900/50">
                  <th className="px-4 py-3 font-medium">Username</th>
                  <th className="px-4 py-3 font-medium">Email</th>
                  <th className="px-4 py-3 font-medium">Role</th>
                  <th className="px-4 py-3 font-medium">Level</th>
                  <th className="px-4 py-3 font-medium">XP</th>
                  <th className="px-4 py-3 font-medium">Status</th>
                  <th className="px-4 py-3 font-medium">Joined</th>
                  <th className="px-4 py-3 font-medium">Actions</th>
                </tr>
              </thead>
              <tbody>
                {isLoading ? (
                  <tr>
                    <td colSpan={8} className="px-4 py-8 text-center text-sm text-text-muted">
                      Loading...
                    </td>
                  </tr>
                ) : data?.users.length === 0 ? (
                  <tr>
                    <td colSpan={8} className="px-4 py-8 text-center text-sm text-text-muted">
                      No users found
                    </td>
                  </tr>
                ) : (
                  data?.users.map((user) => (
                    <tr key={user.id} className="border-b border-border/50 last:border-0">
                      <td className="px-4 py-3 text-sm text-text-primary font-medium">
                        {user.username}
                      </td>
                      <td className="px-4 py-3 text-sm text-text-muted">{user.email}</td>
                      <td className="px-4 py-3">
                        <select
                          value={user.role}
                          onChange={(e) =>
                            updateRoleMutation.mutate({ id: user.id, role: e.target.value })
                          }
                          className="bg-dark-700 border border-border rounded px-2 py-1 text-xs text-text-primary focus:outline-none focus:border-primary"
                        >
                          <option value="student">Student</option>
                          <option value="mentor">Mentor</option>
                          <option value="admin">Admin</option>
                        </select>
                      </td>
                      <td className="px-4 py-3 text-sm text-text-primary">{user.level}</td>
                      <td className="px-4 py-3 text-sm text-amber font-code">{user.xp}</td>
                      <td className="px-4 py-3">
                        {user.is_banned ? (
                          <span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-medium bg-red-500/10 text-red-400 border border-red-500/20">
                            Banned
                          </span>
                        ) : user.is_verified ? (
                          <span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-medium bg-green/10 text-green border border-green/20">
                            Verified
                          </span>
                        ) : (
                          <span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-medium bg-amber/10 text-amber border border-amber/20">
                            Unverified
                          </span>
                        )}
                      </td>
                      <td className="px-4 py-3 text-xs text-text-muted">
                        {new Date(user.created_at).toLocaleDateString()}
                      </td>
                      <td className="px-4 py-3">
                        {!user.is_banned && (
                          <button
                            onClick={() => {
                              if (
                                confirm(
                                  `Ban user ${user.username}? This will revoke all their tokens.`,
                                )
                              ) {
                                banMutation.mutate(user.id);
                              }
                            }}
                            className="flex items-center gap-1 px-2 py-1 text-xs text-red-400 hover:bg-red-500/10 rounded transition-colors"
                            aria-label={`Ban ${user.username}`}
                          >
                            <Ban className="w-3 h-3" />
                            Ban
                          </button>
                        )}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>

          {/* Pagination */}
          {totalPages > 1 && (
            <div className="flex items-center justify-between px-4 py-3 border-t border-border">
              <span className="text-xs text-text-muted">
                Page {page} of {totalPages} ({data?.total} users)
              </span>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setPage((p) => Math.max(1, p - 1))}
                  disabled={page === 1}
                  className="p-1.5 rounded text-text-muted hover:text-text-primary disabled:opacity-30 transition-colors"
                  aria-label="Previous page"
                >
                  <ChevronLeft className="w-4 h-4" />
                </button>
                <button
                  onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
                  disabled={page === totalPages}
                  className="p-1.5 rounded text-text-muted hover:text-text-primary disabled:opacity-30 transition-colors"
                  aria-label="Next page"
                >
                  <ChevronRight className="w-4 h-4" />
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </AdminLayout>
  );
}
