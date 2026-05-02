export default function FinalCTA() {
  return (
    <div className="py-[120px] px-5 md:px-10 text-center relative overflow-hidden bg-dark-950">
      {/* Glow */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[700px] h-[400px] bg-[radial-gradient(ellipse,rgba(206,66,43,0.15)_0%,transparent_70%)] pointer-events-none" />

      <div className="text-[11px] font-bold tracking-[2px] uppercase text-primary mb-3 inline-block relative">
        Join 500,000+ Rustaceans
      </div>
      <h2 className="font-display text-[clamp(40px,5vw,68px)] font-extrabold leading-tight mb-4 relative">
        Start coding Rust
        <br />
        <span className="text-primary">from day one.</span>
      </h2>
      <p className="text-lg text-text-secondary mb-10 relative">
        No setup. No friction. Just you, the borrow checker, and a great adventure.
      </p>
      <div className="flex justify-center gap-4 relative flex-wrap">
        <a
          href="#"
          className="bg-primary text-white px-9 py-4 rounded-xl text-base font-semibold transition-all hover:bg-primary-light hover:-translate-y-0.5 hover:shadow-[0_8px_32px_rgba(206,66,43,0.35)] inline-flex items-center gap-2"
        >
          🦀 Join RustGym for Free
        </a>
        <a
          href="#"
          className="text-text-secondary text-[15px] font-medium inline-flex items-center gap-1.5 transition-colors hover:text-text-primary"
        >
          View curriculum →
        </a>
      </div>
    </div>
  );
}
