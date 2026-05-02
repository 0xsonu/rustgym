import { useQuery } from '@tanstack/react-query';
import { leaderboardApi } from '@/services/api';

export function useLeaderboard(period: 'alltime' | 'weekly' | 'monthly' = 'alltime') {
  return useQuery({
    queryKey: ['leaderboard', period],
    queryFn: () => leaderboardApi.get(period),
    staleTime: 60_000,
  });
}
