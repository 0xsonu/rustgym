import { useQuery } from '@tanstack/react-query';
import { articlesApi } from '@/services/api';

export function useArticles(params?: { page?: number; per_page?: number }) {
  return useQuery({
    queryKey: ['articles', params],
    queryFn: () => articlesApi.list(params),
    staleTime: 30_000,
  });
}

export function useArticle(id: string) {
  return useQuery({
    queryKey: ['article', id],
    queryFn: () => articlesApi.get(id),
    enabled: !!id,
  });
}
