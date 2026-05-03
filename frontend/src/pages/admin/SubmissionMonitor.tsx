import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { RefreshCw, ChevronLeft, ChevronRight } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import { Button, Card, Badge } from '@/components/ui';
import AdminLayout from './AdminLayout';

const statusVariantMap: Record<string, 'success' | 'error' | 'warning' | 'info' | 'default'> = {
  passed: 'success',
  failed: 'error',
  error: 'warning',
  timeout: 'default',
  pending: 'info',
  running: 'info',
};

export default function SubmissionMonitor() {
  const [page, setPage] = useState(1);
  const perPage = 20;

  const { data, isLoading, refetch } = useQuery({
    queryKey: ['admin', 'submissions', { page }],
    queryFn: () => adminApi.listSubmissions({ page, per_page: perPage }),
    refetchInterval: 10000, // Auto-refresh every 10s
  });

  const totalPages = data ? Math.ceil(data.total / perPage) : 0;

  return (
    <AdminLayout>
      <div>
        <div className="flex items-center justify-between mb-6">
          <h1 className="font-display text-2xl font-bold text-text-primary">Submission Monitor</h1>
          <Button variant="secondary" size="sm" onClick={() => void refetch()}>
            <RefreshCw className="w-4 h-4" />
            Refresh
          </Button>
        </div>

        {/* Submissions Table */}
        <Card className="p-0 overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="text-left text-xs text-text-muted border-b border-border bg-surface-base/50">
                  <th className="px-4 py-3 font-medium">ID</th>
                  <th className="px-4 py-3 font-medium">Status</th>
                  <th className="px-4 py-3 font-medium">Duration</th>
                  <th className="px-4 py-3 font-medium">Memory</th>
                  <th className="px-4 py-3 font-medium">XP</th>
                  <th className="px-4 py-3 font-medium">Attempt</th>
                  <th className="px-4 py-3 font-medium">Submitted</th>
                </tr>
              </thead>
              <tbody>
                {isLoading ? (
                  <tr>
                    <td
                      colSpan={7}
                      className="px-4 py-8 text-center text-sm text-text-muted font-body"
                    >
                      Loading...
                    </td>
                  </tr>
                ) : data?.submissions.length === 0 ? (
                  <tr>
                    <td
                      colSpan={7}
                      className="px-4 py-8 text-center text-sm text-text-muted font-body"
                    >
                      No submissions yet
                    </td>
                  </tr>
                ) : (
                  data?.submissions.map((sub, index) => (
                    <tr
                      key={sub.id}
                      className={`border-b border-border/50 last:border-0 ${
                        index % 2 === 1 ? 'bg-surface-base/30' : ''
                      }`}
                    >
                      <td className="px-4 py-3 text-xs text-text-muted font-code">
                        {sub.id.slice(0, 8)}...
                      </td>
                      <td className="px-4 py-3">
                        <Badge variant={statusVariantMap[sub.status] ?? 'default'}>
                          {sub.status}
                        </Badge>
                      </td>
                      <td className="px-4 py-3 text-sm text-text-primary font-body tabular-nums">
                        {sub.duration_ms}ms
                      </td>
                      <td className="px-4 py-3 text-sm text-text-primary font-body tabular-nums">
                        {(sub.memory_kb / 1024).toFixed(1)}MB
                      </td>
                      <td className="px-4 py-3 text-sm text-warning font-code tabular-nums">
                        {sub.xp_awarded > 0 ? `+${sub.xp_awarded}` : '—'}
                      </td>
                      <td className="px-4 py-3 text-sm text-text-muted font-body">
                        #{sub.attempt_number}
                      </td>
                      <td className="px-4 py-3 text-xs text-text-muted font-body">
                        {new Date(sub.created_at).toLocaleString()}
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
                Page {page} of {totalPages} ({data?.total} submissions)
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

        <p className="text-xs text-text-muted mt-3 font-body">Auto-refreshes every 10 seconds</p>
      </div>
    </AdminLayout>
  );
}
