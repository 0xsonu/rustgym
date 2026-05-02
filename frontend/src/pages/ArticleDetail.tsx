import { useParams, Link } from 'react-router-dom';
import { ArrowLeft, Eye, Heart, Calendar } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import { useArticle } from '@/hooks';
import Navbar from '@/components/layout/Navbar';

export default function ArticleDetail() {
  const { id } = useParams<{ id: string }>();
  const { data: article, isLoading } = useArticle(id ?? '');

  if (isLoading) {
    return (
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto">
              <div className="bg-dark-card border border-border rounded-2xl p-8 animate-pulse h-96" />
            </div>
          </section>
        </main>
      </div>
    );
  }

  if (!article) {
    return (
      <div className="min-h-screen bg-dark-950">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto text-center">
              <p className="text-text-secondary">Article not found.</p>
              <Link to="/articles" className="text-primary text-sm mt-4 inline-block">
                ← Back to Articles
              </Link>
            </div>
          </section>
        </main>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-dark-950">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[800px] mx-auto">
            <Link
              to="/articles"
              className="inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-text-primary mb-6 transition-colors"
            >
              <ArrowLeft className="w-4 h-4" />
              Back to Articles
            </Link>

            {/* Cover Image */}
            {article.cover_image_url && (
              <div className="rounded-2xl overflow-hidden mb-8">
                <img
                  src={article.cover_image_url}
                  alt={article.title}
                  className="w-full h-64 object-cover"
                />
              </div>
            )}

            {/* Article Header */}
            <div className="mb-8">
              <h1 className="font-display text-3xl md:text-4xl font-extrabold text-text-primary mb-4">
                {article.title}
              </h1>
              <div className="flex items-center gap-4 text-sm text-text-muted">
                <span className="text-text-secondary font-medium">{article.author_username}</span>
                <span className="flex items-center gap-1">
                  <Calendar className="w-3.5 h-3.5" />
                  {new Date(article.created_at).toLocaleDateString()}
                </span>
                <span className="flex items-center gap-1">
                  <Eye className="w-3.5 h-3.5" />
                  {article.views}
                </span>
                <span className="flex items-center gap-1">
                  <Heart className="w-3.5 h-3.5" />
                  {article.likes}
                </span>
              </div>
              {article.tags.length > 0 && (
                <div className="flex flex-wrap gap-2 mt-4">
                  {article.tags.map((tag) => (
                    <span
                      key={tag}
                      className="px-2.5 py-1 bg-dark-700 border border-border rounded-lg text-xs text-text-secondary"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </div>

            {/* Article Body */}
            <div className="bg-dark-card border border-border rounded-2xl p-8">
              <div className="prose prose-invert prose-lg max-w-none text-text-secondary">
                <ReactMarkdown>{article.body_md}</ReactMarkdown>
              </div>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
}
