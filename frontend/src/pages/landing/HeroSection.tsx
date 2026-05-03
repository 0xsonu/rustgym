import { Link } from 'react-router-dom';
import { Bug, Rocket, Star, Heart, ArrowRight } from 'lucide-react';
import { Button } from '@/components/ui';

export default function HeroSection() {
  return (
    <section className="min-h-screen pt-[120px] pb-20 px-5 md:px-10 flex items-center relative overflow-hidden">
      {/* Grid background */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          backgroundImage:
            'linear-gradient(rgba(206,66,43,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(206,66,43,0.04) 1px, transparent 1px)',
          backgroundSize: '60px 60px',
        }}
      />
      {/* Glow */}
      <div className="absolute -top-[100px] left-1/2 -translate-x-[60%] w-[900px] h-[600px] bg-[radial-gradient(ellipse,rgba(206,66,43,0.12)_0%,transparent_70%)] pointer-events-none" />

      <div className="max-w-[1200px] mx-auto grid grid-cols-1 lg:grid-cols-2 gap-20 items-center relative z-[1] w-full">
        {/* Left column */}
        <div>
          <div className="inline-flex items-center gap-2 bg-primary/[0.12] border border-primary/30 text-primary-light text-xs font-semibold px-3 py-1 rounded-full tracking-wider uppercase mb-5">
            <Bug className="w-3.5 h-3.5" aria-hidden="true" />
            #1 Rust Learning Platform
          </div>
          <h1 className="font-display text-[clamp(32px,6vw,76px)] font-extrabold leading-none tracking-tight mb-2">
            Learn
            <br />
            <span className="text-primary">Rust</span> Online
            <br />
            <span className="text-amber">the Fun Way</span>
          </h1>
          <p className="text-[17px] text-text-secondary max-w-[480px] mb-9 leading-relaxed font-body">
            Master ownership, borrowing, lifetimes, and fearless concurrency through 1,400+ hands-on
            coding tasks. From &ldquo;Hello, world!&rdquo; to systems-level mastery — at your pace,
            with instant feedback.
          </p>
          <div className="flex items-center gap-4 mb-11 flex-wrap">
            <Button asChild size="lg">
              <Link to="/register">
                <Rocket className="w-4 h-4" aria-hidden="true" />
                Start Learning Free
              </Link>
            </Button>
            <Link
              to="/quests"
              className="text-text-secondary text-sm font-medium inline-flex items-center gap-1.5 transition-colors hover:text-text-primary"
            >
              See the curriculum
              <ArrowRight className="w-4 h-4" aria-hidden="true" />
            </Link>
          </div>
          <div className="flex items-center gap-5 flex-wrap">
            <HeroBadge score="4.9" source="G2 Rating" count="1,200+ reviews" />
            <HeroBadge score="4.8" source="Trustpilot" count="800+ reviews" />
            <div className="flex items-center gap-2 bg-surface-elevated border border-border rounded-lg px-3.5 py-2">
              <Heart className="w-5 h-5 text-primary" aria-hidden="true" />
              <div>
                <div className="text-xs font-semibold">Most Loved</div>
                <div className="text-[11px] text-text-muted">9 years in a row</div>
              </div>
            </div>
          </div>
        </div>

        {/* Right column - Terminal */}
        <div className="bg-surface-base border border-border rounded-[14px] overflow-hidden shadow-lg">
          <div className="bg-surface-elevated px-4 py-3 flex items-center gap-2 border-b border-border">
            <div className="w-3 h-3 rounded-full bg-[#ff5f56]" />
            <div className="w-3 h-3 rounded-full bg-[#ffbd2e]" />
            <div className="w-3 h-3 rounded-full bg-[#27c93f]" />
            <div className="flex-1 text-center font-code text-xs text-text-muted">
              main.rs — RustGym IDE
            </div>
          </div>
          <pre className="p-6 font-code text-[13.5px] leading-[1.8] overflow-x-auto">
            <code>
              <span className="text-[#5a6a6a]">// Quest 3 • Ownership &amp; Borrowing</span>
              {'\n'}
              <span className="text-[#5a6a6a]">// Task: Implement a safe string processor</span>
              {'\n\n'}
              <span className="text-[#c678dd]">use</span>{' '}
              <span className="text-[#e5c07b]">std</span>
              <span className="text-text-muted">::</span>
              <span className="text-[#e5c07b]">collections</span>
              <span className="text-text-muted">::</span>
              <span className="text-[#e5c07b]">HashMap</span>
              <span className="text-text-muted">;</span>
              {'\n\n'}
              <span className="text-[#c678dd]">fn</span>{' '}
              <span className="text-[#61afef]">word_count</span>
              <span className="text-text-muted">{'<'}</span>
              <span className="text-primary-light">{`'a`}</span>
              <span className="text-text-muted">{'>'}</span>
              <span className="text-text-muted">(</span>
              {'\n'}
              {'    '}
              <span className="text-text-primary">text</span>
              <span className="text-text-muted">{`: &`}</span>
              <span className="text-primary-light">{`'a`}</span>{' '}
              <span className="text-[#e5c07b]">str</span>
              {'\n'}
              <span className="text-text-muted">{`) -> `}</span>
              <span className="text-[#e5c07b]">HashMap</span>
              <span className="text-text-muted">{`<&`}</span>
              <span className="text-primary-light">{`'a`}</span>{' '}
              <span className="text-[#e5c07b]">str</span>
              <span className="text-text-muted">{`, `}</span>
              <span className="text-[#e5c07b]">usize</span>
              <span className="text-text-muted">{`> {`}</span>
              {'\n'}
              {'    '}
              <span className="text-[#c678dd]">let mut</span>{' '}
              <span className="text-text-primary">map</span>{' '}
              <span className="text-text-muted">= </span>
              <span className="text-[#e5c07b]">HashMap</span>
              <span className="text-text-muted">::</span>
              <span className="text-[#61afef]">new</span>
              <span className="text-text-muted">();</span>
              {'\n'}
              {'    '}
              <span className="text-[#c678dd]">for</span>{' '}
              <span className="text-text-primary">word</span>{' '}
              <span className="text-[#c678dd]">in</span>{' '}
              <span className="text-text-primary">text</span>
              <span className="text-text-muted">.</span>
              <span className="text-[#61afef]">split_whitespace</span>
              <span className="text-text-muted">{`() {`}</span>
              {'\n'}
              {'        '}
              <span className="text-[#c678dd]">let</span>{' '}
              <span className="text-text-primary">count</span>{' '}
              <span className="text-text-muted">= </span>
              <span className="text-text-primary">map</span>
              <span className="text-text-muted">.</span>
              <span className="text-[#61afef]">entry</span>
              <span className="text-text-muted">(</span>
              <span className="text-text-primary">word</span>
              <span className="text-text-muted">).</span>
              <span className="text-[#61afef]">or_insert</span>
              <span className="text-text-muted">(</span>
              <span className="text-[#d19a66]">0</span>
              <span className="text-text-muted">);</span>
              {'\n'}
              {'        '}
              <span className="text-text-muted">*</span>
              <span className="text-text-primary">count</span>{' '}
              <span className="text-text-muted">+= </span>
              <span className="text-[#d19a66]">1</span>
              <span className="text-text-muted">;</span>
              {'\n'}
              {'    '}
              <span className="text-text-muted">{'}'}</span>
              {'\n'}
              {'    '}
              <span className="text-text-primary">map</span>
              {'\n'}
              <span className="text-text-muted">{'}'}</span>
              {'\n\n'}
              <span className="text-primary">$ cargo run</span>
              {'\n'}
              <span className="text-success">✓ Compiling rustgym_task v0.1.0</span>
              {'\n'}
              <span className="text-success">✓ Finished &amp; running in 0.3s</span>
              {'\n'}
              <span className="text-success">✓ Task Complete! +150 XP earned</span>
            </code>
          </pre>
        </div>
      </div>
    </section>
  );
}

function HeroBadge({ score, source, count }: { score: string; source: string; count: string }) {
  return (
    <div className="flex items-center gap-2 bg-surface-elevated border border-border rounded-lg px-3.5 py-2">
      <div>
        <div className="flex items-center gap-0.5 text-amber">
          {[...Array(5)].map((_, i) => (
            <Star key={i} className="w-3 h-3 fill-current" aria-hidden="true" />
          ))}
        </div>
        <div className="text-base font-bold text-text-primary">{score}</div>
      </div>
      <div>
        <div className="text-xs font-semibold">{source}</div>
        <div className="text-[11px] text-text-muted">{count}</div>
      </div>
    </div>
  );
}
