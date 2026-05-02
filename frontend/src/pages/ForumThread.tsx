import { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { ArrowLeft, ThumbsUp, Check, MessageSquare } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import { useForumPost, useCreateReply, useVotePost, useAcceptReply } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';

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
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto">
              <div className="bg-dark-card border border-border rounded-2xl p-8 animate-pulse h-64" />
            </div>
          </section>
        </main>
      </div>
    );
  }

  if (!post) {
    return (
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto text-center">
              <p className="text-text-secondary">Post not found.</p>
              <Link to="/forum" className="text-primary text-sm mt-4 inline-block">
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
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[800px] mx-auto">
            <Link
              to="/forum"
              className="inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-text-primary mb-6 transition-colors"
            >
              <ArrowLeft className="w-4 h-4" />
              Back to Forum
            </Link>

            {/* Post */}
            <div className="bg-dark-card border border-border rounded-2xl p-6 mb-6">
              <div className="flex items-center gap-3 mb-4">
                <span className="px-2 py-0.5 bg-dark-700 rounded text-xs text-text-secondary">
                  {post.category.replace('_', ' ')}
                </span>
                <span className="text-xs text-text-muted">
                  {new Date(post.created_at).toLocaleDateString()}
                </span>
              </div>
              <h1 className="font-display text-2xl font-bold text-text-primary mb-2">
                {post.title}
              </h1>
              <p className="text-xs text-text-muted mb-4">
                by <span className="text-text-secondary font-medium">{post.username}</span>
              </p>
              <div className="prose prose-invert prose-sm max-w-none text-text-secondary">
                <ReactMarkdown>{post.body_md}</ReactMarkdown>
              </div>
              <div className="flex items-center gap-4 mt-6 pt-4 border-t border-border">
                {isAuthenticated && (
                  <button
                    onClick={() => votePost.mutate(post.id)}
                    className="flex items-center gap-1.5 text-xs text-text-muted hover:text-primary transition-colors"
                  >
                    <ThumbsUp className="w-4 h-4" />
                    {post.votes}
                  </button>
                )}
                {!isAuthenticated && (
                  <span className="flex items-center gap-1.5 text-xs text-text-muted">
                    <ThumbsUp className="w-4 h-4" />
                    {post.votes}
                  </span>
                )}
                <span className="flex items-center gap-1.5 text-xs text-text-muted">
                  <MessageSquare className="w-4 h-4" />
                  {post.reply_count} replies
                </span>
              </div>
            </div>

            {/* Replies */}
            <div className="space-y-4 mb-8">
              <h2 className="text-sm font-bold text-text-primary uppercase tracking-wide">
                Replies ({post.replies.length})
              </h2>
              {post.replies.length === 0 ? (
                <p className="text-sm text-text-muted py-4">No replies yet.</p>
              ) : (
                post.replies.map((reply) => (
                  <div
                    key={reply.id}
                    className={`bg-dark-card border rounded-xl p-5 ${
                      reply.is_accepted ? 'border-green-500/40' : 'border-border'
                    }`}
                  >
                    <div className="flex items-center gap-2 mb-3">
                      <span className="text-xs font-medium text-text-secondary">
                        {reply.username}
                      </span>
                      <span className="text-xs text-text-muted">
                        {new Date(reply.created_at).toLocaleDateString()}
                      </span>
                      {reply.is_accepted && (
                        <span className="flex items-center gap-1 text-xs text-green-400 font-medium">
                          <Check className="w-3.5 h-3.5" />
                          Accepted
                        </span>
                      )}
                    </div>
                    <div className="prose prose-invert prose-sm max-w-none text-text-secondary">
                      <ReactMarkdown>{reply.body_md}</ReactMarkdown>
                    </div>
                    <div className="flex items-center gap-3 mt-3 pt-3 border-t border-border">
                      <span className="flex items-center gap-1 text-xs text-text-muted">
                        <ThumbsUp className="w-3.5 h-3.5" />
                        {reply.votes}
                      </span>
                      {isPostAuthor && !reply.is_accepted && (
                        <button
                          onClick={() => acceptReply.mutate(reply.id)}
                          className="text-xs text-green-400 hover:text-green-300 font-medium transition-colors"
                        >
                          Accept Answer
                        </button>
                      )}
                    </div>
                  </div>
                ))
              )}
            </div>

            {/* Reply Form */}
            {isAuthenticated && (
              <div className="bg-dark-card border border-border rounded-2xl p-6">
                <h3 className="text-sm font-bold text-text-primary mb-3">Write a Reply</h3>
                <textarea
                  placeholder="Write your reply in Markdown..."
                  value={replyBody}
                  onChange={(e) => setReplyBody(e.target.value)}
                  rows={4}
                  className="w-full bg-dark-700 border border-border rounded-lg px-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted mb-3 focus:outline-none focus:border-primary resize-none"
                />
                <button
                  onClick={handleReply}
                  disabled={createReply.isPending || !replyBody.trim()}
                  className="bg-primary text-white px-5 py-2 rounded-lg text-sm font-semibold transition-all hover:bg-primary-light disabled:opacity-50"
                >
                  {createReply.isPending ? 'Posting...' : 'Reply'}
                </button>
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
