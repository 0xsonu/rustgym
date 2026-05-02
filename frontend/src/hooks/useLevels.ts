import { useQuery } from '@tanstack/react-query';
import { levelApi } from '@/services/api';

export function useLevelDetail(questSlug: string, levelSlug: string) {
  return useQuery({
    queryKey: ['level', questSlug, levelSlug],
    queryFn: () => levelApi.getBySlug(questSlug, levelSlug),
    enabled: !!questSlug && !!levelSlug,
  });
}
