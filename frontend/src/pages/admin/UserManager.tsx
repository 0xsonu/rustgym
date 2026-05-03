import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Search, Ban, ChevronLeft, ChevronRight } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import { Button, Card, Modal, Badge } from '@/components/ui';
import AdminLayout from './AdminLayout';

export default function UserManager() {
  const queryClient = useQueryClient();
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const [banTarget, setBanTarget] = useState<{ id: string; username: string } | null>(null);
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
      setBanTarget(null);
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
            className="w-full h-10 bg-surface-base border border-border rounded-lg pl-9 pr-4 text-sm text-text-primary font-body placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary"
          />
        </div>

        {/* Users Table */}
        <Card className="p-0 overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="text-left text-xs text-text-muted border-b border-border bg-surface-base/50">
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
                    <td
                      colSpan={8}
                      className="px-4 py-8 text-center text-sm text-text-muted font-body"
                    >
                      Loading...
                    </td>
                  </tr>
                ) : data?.users.length === 0 ? (
                  <tr>
                    <td
                      colSpan={8}
                      className="px-4 py-8 text-center text-sm text-text-muted font-body"
                    >
                      No users found
                    </td>
                  </tr>
                ) : (
                  data?.users.map((user, index) => (
                    <tr
                      key={user.id}
                      className={`border-b border-border/50 last:border-0 ${
                        index % 2 === 1 ? 'bg-surface-base/30' : ''
                      }`}
                    >
                      <td className="px-4 py-3 text-sm text-text-primary font-medium font-body">
                        {user.username}
                      </td>
                      <td className="px-4 py-3 text-sm text-text-muted font-body">{user.email}</td>
                      <td className="px-4 py-3">
                        <select
                          value={user.role}
                          onChange={(e) =>
                            updateRoleMutation.mutate({ id: user.id, role: e.target.value })
                          }
                          className="h-8 bg-surface-base border border-border rounded-md px-2 py-1 text-xs text-text-primary font-body focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary"
                        >
                          <option value="student">Student</option>
                          <option value="mentor">Mentor</option>
                          <option value="admin">Admin</option>
                        </select>
                      </td>
                      <td className="px-4 py-3 text-sm text-text-primary tabular-nums">
                        {user.level}
                      </td>
                      <td className="px-4 py-3 text-sm text-warning font-code tabular-nums">
                        {user.xp}
                      </td>
                      <td className="px-4 py-3">
                        {user.is_banned ? (
                          <Badge variant="error">Banned</Badge>
                        ) : user.is_verified ? (
                          <Badge variant="success">Verified</Badge>
                        ) : (
                          <Badge variant="warning">Unverified</Badge>
                        )}
                      </td>
                      <td className="px-4 py-3 text-xs text-text-muted font-body">
                        {new Date(user.created_at).toLocaleDateString()}
                      </td>
                      <td className="px-4 py-3">
                        {!user.is_banned && (
                          <button
                            onClick={() => setBanTarget({ id: user.id, username: user.username })}
                            className="flex items-center gap-1 px-2 py-1 text-xs text-error hover:bg-error/10 rounded transition-colors duration-200 cursor-pointer"
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
              <span className="text-xs text-text-muted font-body">
                Page {page} of {totalPages} ({data?.total} users)
              </span>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setPage((p) => Math.max(1, p - 1))}
                  disabled={page === 1}
                  className="p-1.5 rounded text-text-muted hover:text-text-primary disabled:opacity-30 transition-colors duration-200 cursor-pointer"
                  aria-label="Previous page"
                >
                  <ChevronLeft className="w-4 h-4" />
                </button>
                <button
                  onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
                  disabled={page === totalPages}
                  className="p-1.5 rounded text-text-muted hover:text-text-primary disabled:opacity-30 transition-colors duration-200 cursor-pointer"
                  aria-label="Next page"
                >
                  <ChevronRight className="w-4 h-4" />
                </button>
              </div>
            </div>
          )}
        </Card>

        {/* Ban Confirmation Modal */}
        <Modal open={banTarget !== null} onClose={() => setBanTarget(null)} title="Ban User">
          <p className="text-sm text-text-secondary font-body mb-6">
            Are you sure you want to ban{' '}
            <span className="font-semibold text-text-primary">{banTarget?.username}</span>? This
            will revoke all their tokens and prevent login.
          </p>
          <div className="flex justify-end gap-3">
            <Button variant="secondary" size="sm" onClick={() => setBanTarget(null)}>
              Cancel
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={() => banTarget && banMutation.mutate(banTarget.id)}
              isLoading={banMutation.isPending}
            >
              Ban User
            </Button>
          </div>
        </Modal>
      </div>
    </AdminLayout>
  );
}
