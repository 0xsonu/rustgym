import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { forumApi } from '@/services/api';

export function useForumPosts(params?: {
  page?: number;
  per_page?: number;
  task_id?: string;
  category?: string;
}) {
  return useQuery({
    queryKey: ['forum-posts', params],
    queryFn: () => forumApi.listPosts(params),
    staleTime: 30_000,
  });
}

export function useForumPost(id: string) {
  return useQuery({
    queryKey: ['forum-post', id],
    queryFn: () => forumApi.getPost(id),
    enabled: !!id,
  });
}

export function useCreateForumPost() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: { title: string; body_md: string; task_id?: string; category?: string }) =>
      forumApi.createPost(data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['forum-posts'] });
    },
  });
}

export function useCreateReply(postId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body_md: string) => forumApi.createReply(postId, body_md),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['forum-post', postId] });
    },
  });
}

export function useVotePost() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (postId: string) => forumApi.votePost(postId),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['forum-posts'] });
    },
  });
}

export function useAcceptReply(postId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (replyId: string) => forumApi.acceptReply(replyId),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['forum-post', postId] });
    },
  });
}
