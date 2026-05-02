import { useQuery } from '@tanstack/react-query';
import { questApi } from '@/services/api';

export function useQuests() {
  return useQuery({
    queryKey: ['quests'],
    queryFn: () => questApi.list(),
  });
}

export function useQuestDetail(slug: string) {
  return useQuery({
    queryKey: ['quest', slug],
    queryFn: () => questApi.getBySlug(slug),
    enabled: !!slug,
  });
}
