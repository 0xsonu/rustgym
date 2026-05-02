import { useState } from 'react';
import { Link } from 'react-router-dom';
import { MessageSquare, Eye, ThumbsUp, Pin, Plus } from 'lucide-react';
import { useForumPosts, useCreateForumPost } from '@/hooks';
import { useAuthStore } from '@/stores/authStore';
import Navbar from '@/components/layout/Navbar';
import type { ForumCategory } from '@/types';

const categories: { label: string; value: ForumCategory | 'all' }[] = [
  { label: 'All', value: 'all' },
  { label: 'General', value: 'general' },
  { label: 'Task Help', value: 'task_help' },
  { label: 'Show & Tell', value: 'show_and_tell' },
];

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
    <div className="min-h-screen bg-dark-950">
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
                <button
                  onClick={() => setShowCreate(!showCreate)}
                  className="flex items-center gap-2 bg-primary text-white px-5 py-2.5 rounded-lg text-sm font-semibold transition-all hover:bg-primary-light hover:-translate-y-px"
                >
                  <Plus className="w-4 h-4" />
                  New Post
                </button>
              )}
            </div>

            {/* Create Post Form */}
            {showCreate && (
              <div className="bg-dark-card border border-border rounded-2xl p-6 mb-8">
                <h3 className="text-lg font-bold text-text-primary mb-4">Create a new post</h3>
                <input
                  type="text"
                  placeholder="Post title..."
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  className="w-full bg-dark-700 border border-border rounded-lg px-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted mb-3 focus:outline-none focus:border-primary"
                />
                <textarea
                  placeholder="Write your post in Markdown..."
                  value={bodyMd}
                  onChange={(e) => setBodyMd(e.target.value)}
                  rows={6}
                  className="w-full bg-dark-700 border border-border rounded-lg px-4 py-2.5 text-sm text-text-primary placeholder:text-text-muted mb-3 focus:outline-none focus:border-primary resize-none"
                />
                <div className="flex items-center gap-3">
                  <select
                    value={postCategory}
                    onChange={(e) => setPostCategory(e.target.value as ForumCategory)}
                    className="bg-dark-700 border border-border rounded-lg px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-primary"
                  >
                    <option value="general">General</option>
                    <option value="task_help">Task Help</option>
                    <option value="show_and_tell">Show &amp; Tell</option>
                  </select>
                  <button
                    onClick={handleCreatePost}
                    disabled={createPost.isPending}
                    className="bg-primary text-white px-5 py-2 rounded-lg text-sm font-semibold transition-all hover:bg-primary-light disabled:opacity-50"
                  >
                    {createPost.isPending ? 'Posting...' : 'Post'}
                  </button>
                </div>
              </div>
            )}

            {/* Category Tabs */}
            <div className="flex gap-1 bg-dark-card border border-border rounded-xl p-1 mb-8 w-fit">
              {categories.map((cat) => (
                <button
                  key={cat.value}
                  onClick={() => {
                    setCategory(cat.value);
                    setPage(1);
                  }}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                    category === cat.value
                      ? 'bg-primary text-white'
                      : 'text-text-secondary hover:text-text-primary hover:bg-dark-700'
                  }`}
                >
                  {cat.label}
                </button>
              ))}
            </div>

            {/* Posts List */}
            {isLoading ? (
              <div className="space-y-3">
                {Array.from({ length: 5 }).map((_, i) => (
                  <div
                    key={i}
                    className="bg-dark-card border border-border rounded-xl p-4 animate-pulse h-20"
                  />
                ))}
              </div>
            ) : posts.length === 0 ? (
              <div className="text-center py-16">
                <MessageSquare className="w-12 h-12 text-text-muted mx-auto mb-4" />
                <p className="text-text-secondary">
                  No posts yet. Be the first to start a discussion!
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {posts.map((post) => (
                  <Link
                    key={post.id}
                    to={`/forum/posts/${post.id}`}
                    className="block bg-dark-card border border-border rounded-xl p-5 transition-all hover:border-primary/40 hover:bg-dark-700"
                  >
                    <div className="flex items-start gap-4">
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2 mb-1">
                          {post.is_pinned && <Pin className="w-3.5 h-3.5 text-amber shrink-0" />}
                          <h3 className="text-sm font-semibold text-text-primary truncate">
                            {post.title}
                          </h3>
                        </div>
                        <div className="flex items-center gap-3 text-xs text-text-muted">
                          <span>{post.username}</span>
                          <span className="px-2 py-0.5 bg-dark-700 rounded text-text-secondary">
                            {post.category.replace('_', ' ')}
                          </span>
                          <span>{new Date(post.created_at).toLocaleDateString()}</span>
                        </div>
                      </div>
                      <div className="flex items-center gap-4 text-xs text-text-muted shrink-0">
                        <span className="flex items-center gap-1">
                          <ThumbsUp className="w-3.5 h-3.5" />
                          {post.votes}
                        </span>
                        <span className="flex items-center gap-1">
                          <MessageSquare className="w-3.5 h-3.5" />
                          {post.reply_count}
                        </span>
                        <span className="flex items-center gap-1">
                          <Eye className="w-3.5 h-3.5" />
                          {post.views}
                        </span>
                      </div>
                    </div>
                  </Link>
                ))}
              </div>
            )}

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="flex items-center justify-center gap-2 mt-8">
                <button
                  onClick={() => setPage((p) => Math.max(1, p - 1))}
                  disabled={page === 1}
                  className="px-4 py-2 rounded-lg text-sm font-medium text-text-secondary hover:text-text-primary hover:bg-dark-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Previous
                </button>
                <span className="text-sm text-text-muted">
                  Page {page} of {totalPages}
                </span>
                <button
                  onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
                  disabled={page === totalPages}
                  className="px-4 py-2 rounded-lg text-sm font-medium text-text-secondary hover:text-text-primary hover:bg-dark-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Next
                </button>
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
