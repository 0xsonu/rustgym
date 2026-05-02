import { useQuery } from '@tanstack/react-query';
import { achievementApi } from '@/services/api';

export function useAchievements() {
  return useQuery({
    queryKey: ['achievements'],
    queryFn: () => achievementApi.getUserAchievements(),
  });
}
