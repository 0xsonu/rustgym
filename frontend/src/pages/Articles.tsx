import { useState } from 'react';
import { Link } from 'react-router-dom';
import { Eye, Heart, BookOpen, Clock, User } from 'lucide-react';
import { useArticles } from '@/hooks';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton, Button } from '@/components/ui';

function ArticlesSkeleton() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {Array.from({ length: 6 }).map((_, i) => (
        <Card key={i} className="p-0 overflow-hidden">
          <Skeleton className="h-40 w-full rounded-none" />
          <div className="p-5 space-y-3">
            <Skeleton className="h-4 w-3/4" />
            <Skeleton className="h-3 w-full" />
            <Skeleton className="h-3 w-1/2" />
            <div className="flex gap-2">
              <Skeleton className="h-5 w-12 rounded-full" />
              <Skeleton className="h-5 w-12 rounded-full" />
            </div>
          </div>
        </Card>
      ))}
    </div>
  );
}

function estimateReadingTime(body?: string): string {
  if (!body) return '3 min read';
  const words = body.split(/\s+/).length;
  const minutes = Math.max(1, Math.ceil(words / 200));
  return `${minutes} min read`;
}

export default function Articles() {
  const [page, setPage] = useState(1);
  const { data, isLoading } = useArticles({ page, per_page: 12 });

  const articles = data?.articles ?? [];
  const total = data?.total ?? 0;
  const totalPages = Math.ceil(total / 12);

  return (
    <div className="min-h-screen bg-surface-base">
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
              <ArticlesSkeleton />
            ) : articles.length === 0 ? (
              <div className="text-center py-16">
                <BookOpen className="w-12 h-12 text-text-muted mx-auto mb-4" aria-hidden="true" />
                <p className="text-text-secondary font-body">No articles published yet.</p>
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {articles.map((article) => (
                  <Link key={article.id} to={`/articles/${article.id}`} className="block group">
                    <Card interactive className="p-0 overflow-hidden h-full">
                      {article.cover_image_url ? (
                        <div className="h-40 overflow-hidden">
                          <img
                            src={article.cover_image_url}
                            alt={article.title}
                            className="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                          />
                        </div>
                      ) : (
                        <div className="h-40 bg-gradient-to-br from-primary/20 to-surface-overlay flex items-center justify-center">
                          <BookOpen className="w-10 h-10 text-primary/60" aria-hidden="true" />
                        </div>
                      )}
                      <div className="p-5">
                        <h3 className="font-display text-sm font-semibold text-text-primary mb-2 line-clamp-2 group-hover:text-primary transition-colors">
                          {article.title}
                        </h3>
                        <div className="flex items-center gap-3 text-xs text-text-muted mb-3 flex-wrap">
                          <span className="flex items-center gap-1">
                            <User className="w-3 h-3" aria-hidden="true" />
                            {article.author_username}
                          </span>
                          <span className="flex items-center gap-1">
                            <Clock className="w-3 h-3" aria-hidden="true" />
                            {new Date(article.created_at).toLocaleDateString()}
                          </span>
                          <span className="flex items-center gap-1">
                            <BookOpen className="w-3 h-3" aria-hidden="true" />
                            {estimateReadingTime()}
                          </span>
                        </div>
                        {article.tags.length > 0 && (
                          <div className="flex flex-wrap gap-1.5 mb-3">
                            {article.tags.slice(0, 3).map((tag) => (
                              <Badge key={tag} variant="default" className="text-[11px]">
                                {tag}
                              </Badge>
                            ))}
                          </div>
                        )}
                        <div className="flex items-center gap-3 text-xs text-text-muted pt-3 border-t border-border">
                          <span className="flex items-center gap-1">
                            <Eye className="w-3.5 h-3.5" aria-hidden="true" />
                            {article.views}
                          </span>
                          <span className="flex items-center gap-1">
                            <Heart className="w-3.5 h-3.5" aria-hidden="true" />
                            {article.likes}
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
              <div className="flex items-center justify-center gap-2 mt-10">
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
