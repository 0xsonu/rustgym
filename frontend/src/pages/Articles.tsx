import { useState } from 'react';
import { Link } from 'react-router-dom';
import { Eye, Heart, BookOpen } from 'lucide-react';
import { useArticles } from '@/hooks';
import Navbar from '@/components/layout/Navbar';

export default function Articles() {
  const [page, setPage] = useState(1);
  const { data, isLoading } = useArticles({ page, per_page: 12 });

  const articles = data?.articles ?? [];
  const total = data?.total ?? 0;
  const totalPages = Math.ceil(total / 12);

  return (
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[1200px] mx-auto">
            <p className="text-xs font-bold tracking-[2px] uppercase text-primary mb-3">
              COMMUNITY
            </p>
            <h1 className="font-display text-4xl md:text-5xl font-extrabold text-text-primary mb-8">
              Articles
            </h1>

            {isLoading ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {Array.from({ length: 6 }).map((_, i) => (
                  <div
                    key={i}
                    className="bg-dark-card border border-border rounded-2xl animate-pulse h-72"
                  />
                ))}
              </div>
            ) : articles.length === 0 ? (
              <div className="text-center py-16">
                <BookOpen className="w-12 h-12 text-text-muted mx-auto mb-4" />
                <p className="text-text-secondary">No articles published yet.</p>
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {articles.map((article) => (
                  <Link
                    key={article.id}
                    to={`/articles/${article.id}`}
                    className="group bg-dark-card border border-border rounded-2xl overflow-hidden transition-all hover:border-primary/40 hover:-translate-y-1"
                  >
                    {article.cover_image_url ? (
                      <div className="h-40 overflow-hidden">
                        <img
                          src={article.cover_image_url}
                          alt={article.title}
                          className="w-full h-full object-cover transition-transform group-hover:scale-105"
                        />
                      </div>
                    ) : (
                      <div className="h-40 bg-gradient-to-br from-primary/20 to-dark-700 flex items-center justify-center">
                        <BookOpen className="w-10 h-10 text-primary/60" />
                      </div>
                    )}
                    <div className="p-5">
                      <h3 className="text-sm font-semibold text-text-primary mb-2 line-clamp-2 group-hover:text-primary transition-colors">
                        {article.title}
                      </h3>
                      <p className="text-xs text-text-muted mb-3">
                        by {article.author_username} ·{' '}
                        {new Date(article.created_at).toLocaleDateString()}
                      </p>
                      {article.tags.length > 0 && (
                        <div className="flex flex-wrap gap-1.5 mb-3">
                          {article.tags.slice(0, 3).map((tag) => (
                            <span
                              key={tag}
                              className="px-2 py-0.5 bg-dark-700 rounded text-[11px] text-text-secondary"
                            >
                              {tag}
                            </span>
                          ))}
                        </div>
                      )}
                      <div className="flex items-center gap-3 text-xs text-text-muted">
                        <span className="flex items-center gap-1">
                          <Eye className="w-3.5 h-3.5" />
                          {article.views}
                        </span>
                        <span className="flex items-center gap-1">
                          <Heart className="w-3.5 h-3.5" />
                          {article.likes}
                        </span>
                      </div>
                    </div>
                  </Link>
                ))}
              </div>
            )}

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="flex items-center justify-center gap-2 mt-10">
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
