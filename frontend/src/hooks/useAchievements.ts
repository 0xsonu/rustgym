import { useQuery } from '@tanstack/react-query';
import { achievementApi } from '@/services/api';
import { useAuthStore } from '@/stores/authStore';

export function useAchievements(enabled = true) {
  const isAuthenticated = useAuthStore((s) => s.isAuthenticated);

  return useQuery({
    queryKey: ['achievements'],
    queryFn: () => achievementApi.getUserAchievements(),
    enabled: enabled && isAuthenticated,
    retry: false, // Don't retry on 401
  });
}
