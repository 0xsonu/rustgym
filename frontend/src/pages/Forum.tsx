import { useState } from 'react';
import { Link } from 'react-router-dom';
import { MessageSquare, Eye, ThumbsUp, Pin, Plus, Clock, User } from 'lucide-react';
import { useForumPosts, useCreateForumPost } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton, Button } from '@/components/ui';
import type { ForumCategory } from '@/types';

const categories: { label: string; value: ForumCategory | 'all' }[] = [
  { label: 'All', value: 'all' },
  { label: 'General', value: 'general' },
  { label: 'Task Help', value: 'task_help' },
  { label: 'Show & Tell', value: 'show_and_tell' },
];

function ForumSkeleton() {
  return (
    <div className="space-y-3">
      {Array.from({ length: 5 }).map((_, i) => (
        <Card key={i} className="p-5">
          <div className="flex items-start gap-4">
            <Skeleton className="w-10 h-10 rounded-full shrink-0" />
            <div className="flex-1 space-y-2">
              <Skeleton className="h-4 w-3/4" />
              <Skeleton className="h-3 w-1/2" />
            </div>
            <div className="flex gap-4">
              <Skeleton className="h-4 w-10" />
              <Skeleton className="h-4 w-10" />
            </div>
          </div>
        </Card>
      ))}
    </div>
  );
}

function AuthorAvatar({ username }: { username: string }) {
  const initials = username.slice(0, 2).toUpperCase();
  return (
    <div className="w-10 h-10 rounded-full bg-primary/20 border border-primary/40 flex items-center justify-center shrink-0">
      <span className="text-xs font-bold text-primary">{initials}</span>
    </div>
  );
}

export default function Forum() {
  const [category, setCategory] = useState<ForumCategory | 'all'>('all');
  const [page, setPage] = useState(1);
  const [showCreate, setShowCreate] = useState(false);
  const [title, setTitle] = useState('');
  const [bodyMd, setBodyMd] = useState('');
  const [postCategory, setPostCategory] = useState<ForumCategory>('general');

  const { isAuthenticated } = useAuthStore();
  const createPost = useCreateForumPost();

  const queryParams = {
    page,
    per_page: 20,
    ...(category !== 'all' ? { category } : {}),
  };

  const { data, isLoading } = useForumPosts(queryParams);
  const posts = data?.posts ?? [];
  const total = data?.total ?? 0;
  const totalPages = Math.ceil(total / 20);

  const handleCreatePost = () => {
    if (!title.trim() || !bodyMd.trim()) return;
    createPost.mutate(
      { title, body_md: bodyMd, category: postCategory },
      {
        onSuccess: () => {
          setShowCreate(false);
          setTitle('');
          setBodyMd('');
        },
      },
    );
  };

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[1000px] mx-auto">
            <div className="flex items-center justify-between mb-8">
              <div>
                <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-3">
                  COMMUNITY
                </p>
                <h1 className="font-display text-4xl md:text-5xl font-extrabold text-text-primary">
                  Forum
                </h1>
              </div>
              {isAuthenticated && (
                <Button onClick={() => setShowCreate(!showCreate)} variant="primary" size="md">
                  <Plus className="w-4 h-4" aria-hidden="true" />
                  New Post
                </Button>
              )}
            </div>

            {/* Create Post Form */}
            {showCreate && (
              <Card className="mb-8">
                <h3 className="font-display text-lg font-bold text-text-primary mb-4">
                  Create a new post
                </h3>
                <input
                  type="text"
                  placeholder="Post title..."
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  className="w-full bg-surface-base border border-border rounded-lg px-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted mb-3 focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary transition-all"
                />
                <textarea
                  placeholder="Write your post in Markdown..."
                  value={bodyMd}
                  onChange={(e) => setBodyMd(e.target.value)}
                  rows={6}
                  className="w-full bg-surface-base border border-border rounded-lg px-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted mb-3 focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary resize-none transition-all"
                />
                <div className="flex items-center gap-3">
                  <select
                    value={postCategory}
                    onChange={(e) => setPostCategory(e.target.value as ForumCategory)}
                    className="bg-surface-base border border-border rounded-lg px-3 py-2 text-sm text-text-primary focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary transition-all"
                  >
                    <option value="general">General</option>
                    <option value="task_help">Task Help</option>
                    <option value="show_and_tell">Show &amp; Tell</option>
                  </select>
                  <Button
                    onClick={handleCreatePost}
                    isLoading={createPost.isPending}
                    variant="primary"
                    size="md"
                  >
                    Post
                  </Button>
                </div>
              </Card>
            )}

            {/* Category Tabs */}
            <div className="flex gap-1 bg-surface-elevated border border-border rounded-xl p-1 mb-8 w-fit">
              {categories.map((cat) => (
                <button
                  key={cat.value}
                  onClick={() => {
                    setCategory(cat.value);
                    setPage(1);
                  }}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-all cursor-pointer ${
                    category === cat.value
                      ? 'bg-primary text-white'
                      : 'text-text-secondary hover:text-text-primary hover:bg-surface-overlay'
                  }`}
                >
                  {cat.label}
                </button>
              ))}
            </div>

            {/* Posts List */}
            {isLoading ? (
              <ForumSkeleton />
            ) : posts.length === 0 ? (
              <div className="text-center py-16">
                <MessageSquare
                  className="w-12 h-12 text-text-muted mx-auto mb-4"
                  aria-hidden="true"
                />
                <p className="text-text-secondary font-body">
                  No posts yet. Be the first to start a discussion!
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {posts.map((post) => (
                  <Link key={post.id} to={`/forum/posts/${post.id}`} className="block">
                    <Card interactive className="p-5">
                      <div className="flex items-start gap-4">
                        <AuthorAvatar username={post.username} />
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-1">
                            {post.is_pinned && (
                              <Pin
                                className="w-3.5 h-3.5 text-warning shrink-0"
                                aria-hidden="true"
                              />
                            )}
                            <h3 className="text-sm font-semibold text-text-primary truncate">
                              {post.title}
                            </h3>
                          </div>
                          <div className="flex items-center gap-3 text-xs text-text-muted flex-wrap">
                            <span className="flex items-center gap-1">
                              <User className="w-3 h-3" aria-hidden="true" />
                              {post.username}
                            </span>
                            <Badge variant="default" className="text-[11px]">
                              {post.category.replace('_', ' ')}
                            </Badge>
                            <span className="flex items-center gap-1">
                              <Clock className="w-3 h-3" aria-hidden="true" />
                              {new Date(post.created_at).toLocaleDateString()}
                            </span>
                          </div>
                        </div>
                        <div className="flex items-center gap-4 text-xs text-text-muted shrink-0">
                          <span className="flex items-center gap-1">
                            <ThumbsUp className="w-3.5 h-3.5" aria-hidden="true" />
                            {post.votes}
                          </span>
                          <span className="flex items-center gap-1">
                            <MessageSquare className="w-3.5 h-3.5" aria-hidden="true" />
                            {post.reply_count}
                          </span>
                          <span className="flex items-center gap-1">
                            <Eye className="w-3.5 h-3.5" aria-hidden="true" />
                            {post.views}
                          </span>
                        </div>
                      </div>
                    </Card>
                  </Link>
                ))}
              </div>
            )}

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="flex items-center justify-center gap-2 mt-8">
                <Button
                  onClick={() => setPage((p) => Math.max(1, p - 1))}
                  disabled={page === 1}
                  variant="ghost"
                  size="sm"
                >
                  Previous
                </Button>
                <span className="text-sm text-text-muted">
                  Page {page} of {totalPages}
                </span>
                <Button
                  onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
                  disabled={page === totalPages}
                  variant="ghost"
                  size="sm"
                >
                  Next
                </Button>
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
