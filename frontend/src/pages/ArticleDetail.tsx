import { useParams, Link } from 'react-router-dom';
import { ArrowLeft, Eye, Heart, Calendar, Clock, BookOpen, User } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import { useArticle } from '@/hooks';
import Navbar from '@/components/layout/Navbar';
import { Card, Badge, Skeleton } from '@/components/ui';

function ArticleDetailSkeleton() {
  return (
    <div className="max-w-[800px] mx-auto space-y-6">
      <Skeleton className="h-4 w-32" />
      <Skeleton className="h-64 w-full rounded-2xl" />
      <div className="space-y-4">
        <Skeleton className="h-10 w-3/4" />
        <div className="flex gap-4">
          <Skeleton className="h-4 w-24" />
          <Skeleton className="h-4 w-20" />
          <Skeleton className="h-4 w-16" />
        </div>
      </div>
      <Card className="space-y-4 p-8">
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-5/6" />
        <Skeleton className="h-4 w-4/6" />
        <Skeleton className="h-32 w-full" />
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-3/4" />
      </Card>
    </div>
  );
}

function estimateReadingTime(body?: string): string {
  if (!body) return '3 min read';
  const words = body.split(/\s+/).length;
  const minutes = Math.max(1, Math.ceil(words / 200));
  return `${minutes} min read`;
}

export default function ArticleDetail() {
  const { id } = useParams<{ id: string }>();
  const { data: article, isLoading } = useArticle(id ?? '');

  if (isLoading) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <ArticleDetailSkeleton />
          </section>
        </main>
      </div>
    );
  }

  if (!article) {
    return (
      <div className="min-h-screen bg-surface-base">
        <Navbar />
        <main className="pt-[62px]">
          <section className="py-12 px-5 md:px-10">
            <div className="max-w-[800px] mx-auto text-center">
              <BookOpen className="w-12 h-12 text-text-muted mx-auto mb-4" aria-hidden="true" />
              <p className="text-text-secondary font-body">Article not found.</p>
              <Link
                to="/articles"
                className="text-primary text-sm mt-4 inline-block hover:underline"
              >
                ← Back to Articles
              </Link>
            </div>
          </section>
        </main>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-surface-base">
      <Navbar />
      <main className="pt-[62px]">
        <section className="py-12 px-5 md:px-10">
          <div className="max-w-[800px] mx-auto">
            <Link
              to="/articles"
              className="inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-text-primary mb-6 transition-colors"
            >
              <ArrowLeft className="w-4 h-4" aria-hidden="true" />
              Back to Articles
            </Link>

            {/* Cover Image */}
            {article.cover_image_url && (
              <div className="rounded-2xl overflow-hidden mb-8 border border-border">
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
              <div className="flex items-center gap-4 text-sm text-text-muted flex-wrap">
                <span className="flex items-center gap-1.5 text-text-secondary font-medium">
                  <User className="w-3.5 h-3.5" aria-hidden="true" />
                  {article.author_username}
                </span>
                <span className="flex items-center gap-1.5">
                  <Calendar className="w-3.5 h-3.5" aria-hidden="true" />
                  {new Date(article.created_at).toLocaleDateString()}
                </span>
                <span className="flex items-center gap-1.5">
                  <Clock className="w-3.5 h-3.5" aria-hidden="true" />
                  {estimateReadingTime(article.body_md)}
                </span>
                <span className="flex items-center gap-1.5">
                  <Eye className="w-3.5 h-3.5" aria-hidden="true" />
                  {article.views}
                </span>
                <span className="flex items-center gap-1.5">
                  <Heart className="w-3.5 h-3.5" aria-hidden="true" />
                  {article.likes}
                </span>
              </div>
              {article.tags.length > 0 && (
                <div className="flex flex-wrap gap-2 mt-4">
                  {article.tags.map((tag) => (
                    <Badge key={tag} variant="default">
                      {tag}
                    </Badge>
                  ))}
                </div>
              )}
            </div>

            {/* Article Body */}
            <Card className="p-8">
              <div
                className="prose prose-invert prose-lg max-w-none font-body text-text-secondary
                [&_h1]:font-display [&_h1]:text-text-primary [&_h1]:font-extrabold [&_h1]:text-3xl [&_h1]:mb-4 [&_h1]:mt-8
                [&_h2]:font-display [&_h2]:text-text-primary [&_h2]:font-bold [&_h2]:text-2xl [&_h2]:mb-3 [&_h2]:mt-6
                [&_h3]:font-display [&_h3]:text-text-primary [&_h3]:font-semibold [&_h3]:text-xl [&_h3]:mb-2 [&_h3]:mt-5
                [&_h4]:font-display [&_h4]:text-text-primary [&_h4]:font-semibold [&_h4]:text-lg [&_h4]:mb-2 [&_h4]:mt-4
                [&_p]:font-body [&_p]:text-text-secondary [&_p]:leading-relaxed [&_p]:mb-4
                [&_a]:text-primary [&_a]:underline [&_a]:hover:text-primary-400
                [&_strong]:text-text-primary [&_strong]:font-semibold
                [&_ul]:list-disc [&_ul]:pl-6 [&_ul]:mb-4
                [&_ol]:list-decimal [&_ol]:pl-6 [&_ol]:mb-4
                [&_li]:text-text-secondary [&_li]:mb-1
                [&_blockquote]:border-l-4 [&_blockquote]:border-primary/50 [&_blockquote]:pl-4 [&_blockquote]:italic [&_blockquote]:text-text-muted
                [&_code]:font-code [&_code]:text-sm [&_code]:bg-slate-900 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:rounded [&_code]:text-primary-300
                [&_pre]:bg-[#1E293B] [&_pre]:border [&_pre]:border-border [&_pre]:rounded-lg [&_pre]:p-4 [&_pre]:overflow-x-auto [&_pre]:mb-4
                [&_pre_code]:bg-transparent [&_pre_code]:p-0 [&_pre_code]:text-text-secondary [&_pre_code]:text-sm [&_pre_code]:font-code
                [&_hr]:border-border [&_hr]:my-8
                [&_table]:w-full [&_table]:border-collapse
                [&_th]:text-left [&_th]:text-text-primary [&_th]:font-semibold [&_th]:border-b [&_th]:border-border [&_th]:pb-2
                [&_td]:text-text-secondary [&_td]:border-b [&_td]:border-border [&_td]:py-2
                [&_img]:rounded-lg [&_img]:max-w-full
              "
              >
                <ReactMarkdown rehypePlugins={[rehypeHighlight]}>{article.body_md}</ReactMarkdown>
              </div>
            </Card>
          </div>
        </section>
      </main>
    </div>
  );
}
