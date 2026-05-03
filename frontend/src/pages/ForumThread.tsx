import { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { ArrowLeft, ThumbsUp, Check, MessageSquare, Clock } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import { useForumPost, useCreateReply, useVotePost, useAcceptReply } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton, Button } from '@/components/ui';

function AuthorAvatar({ username, size = 'md' }: { username: string; size?: 'sm' | 'md' }) {
  const initials = username.slice(0, 2).toUpperCase();
  const sizeClasses = { sm: 'w-8 h-8 text-[10px]', md: 'w-10 h-10 text-xs' };
  return (
    <div
      className={`${sizeClasses[size]} rounded-full bg-primary/20 border border-primary/40 flex items-center justify-center shrink-0`}
    >
      <span className="font-bold text-primary">{initials}</span>
    </div>
  );
}

function ThreadSkeleton() {
  return (
    <div className="space-y-6">
      <Card className="p-6">
        <div className="space-y-4">
          <Skeleton className="h-6 w-3/4" />
          <div className="flex gap-3">
            <Skeleton className="w-10 h-10 rounded-full" />
            <div className="space-y-2">
              <Skeleton className="h-3 w-24" />
              <Skeleton className="h-3 w-16" />
            </div>
          </div>
          <Skeleton className="h-32 w-full" />
        </div>
      </Card>
      <div className="space-y-3">
        {Array.from({ length: 3 }).map((_, i) => (
          <Card key={i} className="p-5">
            <div className="flex gap-3">
              <Skeleton className="w-8 h-8 rounded-full" />
              <div className="flex-1 space-y-2">
                <Skeleton className="h-3 w-32" />
                <Skeleton className="h-16 w-full" />
              </div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
}

export default function ForumThread() {
  const { id } = useParams<{ id: string }>();
  const { data: post, isLoading } = useForumPost(id ?? '');
  const { isAuthenticated, user } = useAuthStore();
  const [replyBody, setReplyBody] = useState('');

  const createReply = useCreateReply(id ?? '');
  const votePost = useVotePost();
  const acceptReply = useAcceptReply(id ?? '');

  const handleReply = () => {
    if (!replyBody.trim()) return;
    createReply.mutate(replyBody, {
      onSuccess: () => setReplyBody(''),
    });
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto">
              <ThreadSkeleton />
            </div>
          </section>
        </main>
      </div>
    );
  }

  if (!post) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto text-center">
              <MessageSquare
                className="w-12 h-12 text-text-muted mx-auto mb-4"
                aria-hidden="true"
              />
              <p className="text-text-secondary font-body">Post not found.</p>
              <Link to="/forum" className="text-primary text-sm mt-4 inline-block hover:underline">
                ← Back to Forum
              </Link>
            </div>
          </section>
        </main>
      </div>
    );
  }

  const isPostAuthor = user?.id === post.user_id;

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[800px] mx-auto">
            <Link
              to="/forum"
              className="inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-text-primary mb-6 transition-colors"
            >
              <ArrowLeft className="w-4 h-4" aria-hidden="true" />
              Back to Forum
            </Link>

            {/* Original Post */}
            <Card className="mb-6">
              <div className="flex items-center gap-3 mb-4">
                <Badge variant="default">{post.category.replace('_', ' ')}</Badge>
                <span className="flex items-center gap-1 text-xs text-text-muted">
                  <Clock className="w-3 h-3" aria-hidden="true" />
                  {new Date(post.created_at).toLocaleDateString()}
                </span>
              </div>
              <h1 className="font-display text-2xl font-bold text-text-primary mb-4">
                {post.title}
              </h1>
              <div className="flex items-center gap-3 mb-4 pb-4 border-b border-border">
                <AuthorAvatar username={post.username} />
                <div>
                  <p className="text-sm font-medium text-text-primary">{post.username}</p>
                  <p className="text-xs text-text-muted">Author</p>
                </div>
              </div>
              <div className="prose prose-invert prose-sm max-w-none text-text-secondary font-body [&_code]:font-code [&_code]:bg-surface-base [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:rounded [&_pre]:bg-slate-900 [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg">
                <ReactMarkdown>{post.body_md}</ReactMarkdown>
              </div>
              <div className="flex items-center gap-4 mt-6 pt-4 border-t border-border">
                {isAuthenticated ? (
                  <button
                    onClick={() => votePost.mutate(post.id)}
                    className="flex items-center gap-1.5 text-xs text-text-muted hover:text-primary transition-colors cursor-pointer"
                  >
                    <ThumbsUp className="w-4 h-4" aria-hidden="true" />
                    {post.votes}
                  </button>
                ) : (
                  <span className="flex items-center gap-1.5 text-xs text-text-muted">
                    <ThumbsUp className="w-4 h-4" aria-hidden="true" />
                    {post.votes}
                  </span>
                )}
                <span className="flex items-center gap-1.5 text-xs text-text-muted">
                  <MessageSquare className="w-4 h-4" aria-hidden="true" />
                  {post.reply_count} replies
                </span>
              </div>
            </Card>

            {/* Replies Section */}
            <div className="mb-8">
              <h2 className="font-display text-sm font-bold text-text-primary uppercase tracking-wide mb-4">
                Replies ({post.replies.length})
              </h2>
              {post.replies.length === 0 ? (
                <Card className="text-center py-8">
                  <MessageSquare
                    className="w-8 h-8 text-text-muted mx-auto mb-2"
                    aria-hidden="true"
                  />
                  <p className="text-sm text-text-muted font-body">
                    No replies yet. Be the first to respond!
                  </p>
                </Card>
              ) : (
                <div className="space-y-3">
                  {post.replies.map((reply) => (
                    <Card
                      key={reply.id}
                      className={`p-5 ${reply.is_accepted ? 'border-success/40 bg-success/5' : ''}`}
                    >
                      <div className="flex gap-3">
                        <AuthorAvatar username={reply.username} size="sm" />
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-2 flex-wrap">
                            <span className="text-sm font-medium text-text-primary">
                              {reply.username}
                            </span>
                            <span className="flex items-center gap-1 text-xs text-text-muted">
                              <Clock className="w-3 h-3" aria-hidden="true" />
                              {new Date(reply.created_at).toLocaleDateString()}
                            </span>
                            {reply.is_accepted && (
                              <Badge variant="success">
                                <Check className="w-3 h-3" aria-hidden="true" />
                                Accepted
                              </Badge>
                            )}
                          </div>
                          <div className="prose prose-invert prose-sm max-w-none text-text-secondary font-body [&_code]:font-code [&_code]:bg-surface-base [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:rounded [&_pre]:bg-slate-900 [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg">
                            <ReactMarkdown>{reply.body_md}</ReactMarkdown>
                          </div>
                          <div className="flex items-center gap-3 mt-3 pt-3 border-t border-border">
                            <span className="flex items-center gap-1 text-xs text-text-muted">
                              <ThumbsUp className="w-3.5 h-3.5" aria-hidden="true" />
                              {reply.votes}
                            </span>
                            {isPostAuthor && !reply.is_accepted && (
                              <button
                                onClick={() => acceptReply.mutate(reply.id)}
                                className="flex items-center gap-1 text-xs text-success hover:text-green-300 font-medium transition-colors cursor-pointer"
                              >
                                <Check className="w-3.5 h-3.5" aria-hidden="true" />
                                Accept Answer
                              </button>
                            )}
                          </div>
                        </div>
                      </div>
                    </Card>
                  ))}
                </div>
              )}
            </div>

            {/* Reply Form */}
            {isAuthenticated && (
              <Card>
                <h3 className="font-display text-sm font-bold text-text-primary mb-3">
                  Write a Reply
                </h3>
                <textarea
                  placeholder="Write your reply in Markdown..."
                  value={replyBody}
                  onChange={(e) => setReplyBody(e.target.value)}
                  rows={4}
                  className="w-full bg-surface-base border border-border rounded-lg px-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted mb-3 focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary resize-none transition-all font-body"
                />
                <Button
                  onClick={handleReply}
                  isLoading={createReply.isPending}
                  disabled={!replyBody.trim()}
                  variant="primary"
                  size="md"
                >
                  Reply
                </Button>
              </Card>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
