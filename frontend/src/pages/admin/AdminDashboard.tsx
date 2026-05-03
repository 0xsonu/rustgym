import { useQuery } from '@tanstack/react-query';
import { Users, FileText, TrendingUp, Star } from 'lucide-react';
import { adminApi } from '@/services/adminApi';
import { Card } from '@/components/ui';
import { Skeleton } from '@/components/ui';
import AdminLayout from './AdminLayout';

export default function AdminDashboard() {
  const { data: stats, isLoading } = useQuery({
    queryKey: ['admin', 'stats'],
    queryFn: () => adminApi.getStats(),
  });

  return (
    <AdminLayout>
      <div>
        <h1 className="font-display text-2xl font-bold text-text-primary mb-6">Dashboard</h1>

        {isLoading ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {Array.from({ length: 4 }).map((_, i) => (
              <Skeleton key={i} className="h-[100px] w-full rounded-xl" />
            ))}
          </div>
        ) : (
          <>
            {/* Stats Cards */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
              <StatCard
                icon={<Users className="w-5 h-5" />}
                label="Total Users"
                value={stats?.total_users ?? 0}
                color="text-info"
                bgColor="bg-info/10"
              />
              <StatCard
                icon={<FileText className="w-5 h-5" />}
                label="Submissions Today"
                value={stats?.submissions_today ?? 0}
                color="text-warning"
                bgColor="bg-warning/10"
              />
              <StatCard
                icon={<TrendingUp className="w-5 h-5" />}
                label="Pass Rate"
                value={`${(stats?.pass_rate ?? 0).toFixed(1)}%`}
                color="text-success"
                bgColor="bg-success/10"
              />
              <StatCard
                icon={<Star className="w-5 h-5" />}
                label="Popular Tasks"
                value={stats?.popular_tasks?.length ?? 0}
                color="text-primary"
                bgColor="bg-primary/10"
              />
            </div>

            {/* Popular Tasks Table */}
            {stats?.popular_tasks && stats.popular_tasks.length > 0 && (
              <Card className="p-6">
                <h2 className="font-display text-lg font-bold text-text-primary mb-4">
                  Popular Tasks
                </h2>
                <table className="w-full">
                  <thead>
                    <tr className="text-left text-xs text-text-muted border-b border-border">
                      <th className="pb-3 font-medium">Task</th>
                      <th className="pb-3 font-medium">Slug</th>
                      <th className="pb-3 font-medium text-right">Submissions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {stats.popular_tasks.map((task, index) => (
                      <tr
                        key={task.id}
                        className={`border-b border-border/50 last:border-0 ${
                          index % 2 === 1 ? 'bg-surface-base/50' : ''
                        }`}
                      >
                        <td className="py-3 text-sm text-text-primary font-body">{task.title}</td>
                        <td className="py-3 text-sm text-text-muted font-code">{task.slug}</td>
                        <td className="py-3 text-sm text-text-primary text-right tabular-nums">
                          {task.submission_count}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </Card>
            )}
          </>
        )}
      </div>
    </AdminLayout>
  );
}

function StatCard({
  icon,
  label,
  value,
  color,
  bgColor,
}: {
  icon: React.ReactNode;
  label: string;
  value: string | number;
  color: string;
  bgColor: string;
}) {
  return (
    <Card className="p-5">
      <div className="flex items-center gap-3 mb-3">
        <div className={`w-9 h-9 rounded-lg ${bgColor} flex items-center justify-center ${color}`}>
          {icon}
        </div>
        <span className="text-xs text-text-muted font-medium font-body">{label}</span>
      </div>
      <p className="text-2xl font-bold text-text-primary font-display tabular-nums">{value}</p>
    </Card>
  );
}
