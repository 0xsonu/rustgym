import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { RefreshCw, ChevronLeft, ChevronRight } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import AdminLayout from './AdminLayout';

function StatusBadge({ status }: { status: string }) {
  const styles: Record<string, string> = {
    passed: 'bg-green/10 text-green border-green/20',
    failed: 'bg-primary/10 text-primary border-primary/20',
    error: 'bg-amber/10 text-amber border-amber/20',
    timeout: 'bg-text-muted/10 text-text-muted border-text-muted/20',
    pending: 'bg-blue-400/10 text-blue-400 border-blue-400/20',
    running: 'bg-blue-400/10 text-blue-400 border-blue-400/20',
  };

  return (
    <span
      className={`inline-flex items-center px-2 py-0.5 rounded text-[11px] font-medium border ${styles[status] ?? styles.error}`}
    >
      {status}
    </span>
  );
}

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
          <button
            onClick={() => void refetch()}
            className="flex items-center gap-2 px-3 py-2 text-sm text-text-secondary hover:text-text-primary bg-dark-card border border-border rounded-lg transition-colors"
          >
            <RefreshCw className="w-4 h-4" />
            Refresh
          </button>
        </div>

        {/* Submissions Table */}
        <div className="bg-dark-card border border-border rounded-xl overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="text-left text-xs text-text-muted border-b border-border bg-dark-900/50">
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
                    <td colSpan={7} className="px-4 py-8 text-center text-sm text-text-muted">
                      Loading...
                    </td>
                  </tr>
                ) : data?.submissions.length === 0 ? (
                  <tr>
                    <td colSpan={7} className="px-4 py-8 text-center text-sm text-text-muted">
                      No submissions yet
                    </td>
                  </tr>
                ) : (
                  data?.submissions.map((sub) => (
                    <tr key={sub.id} className="border-b border-border/50 last:border-0">
                      <td className="px-4 py-3 text-xs text-text-muted font-code">
                        {sub.id.slice(0, 8)}...
                      </td>
                      <td className="px-4 py-3">
                        <StatusBadge status={sub.status} />
                      </td>
                      <td className="px-4 py-3 text-sm text-text-primary">{sub.duration_ms}ms</td>
                      <td className="px-4 py-3 text-sm text-text-primary">
                        {(sub.memory_kb / 1024).toFixed(1)}MB
                      </td>
                      <td className="px-4 py-3 text-sm text-amber font-code">
                        {sub.xp_awarded > 0 ? `+${sub.xp_awarded}` : '—'}
                      </td>
                      <td className="px-4 py-3 text-sm text-text-muted">#{sub.attempt_number}</td>
                      <td className="px-4 py-3 text-xs text-text-muted">
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
              <span className="text-xs text-text-muted">
                Page {page} of {totalPages} ({data?.total} submissions)
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

        <p className="text-xs text-text-muted mt-3">Auto-refreshes every 10 seconds</p>
      </div>
    </AdminLayout>
  );
}
