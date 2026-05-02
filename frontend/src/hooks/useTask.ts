import { useQuery } from '@tanstack/react-query';
import { taskApi } from '@/services/api';

export function useTaskDetail(slug: string) {
  return useQuery({
    queryKey: ['task', slug],
    queryFn: () => taskApi.getBySlug(slug),
    enabled: !!slug,
  });
}
