import { FadeIn } from '../Landing';

const stats = [
  { num: '500K+', label: 'Registered Rustaceans' },
  { num: '18M+', label: 'Coding Tasks Completed' },
  { num: '1,400+', label: 'Hands-On Exercises' },
  { num: '12K+', label: 'Graduates Employed' },
];

export default function StatsBanner() {
  return (
    <div className="bg-primary-dark">
      <div className="max-w-[1200px] mx-auto py-20 px-5 md:px-10">
        <FadeIn>
          <div className="grid grid-cols-2 lg:grid-cols-4">
            {stats.map((s, i) => (
              <div
                key={s.label}
                className={`text-center py-10 px-5 ${i < stats.length - 1 ? 'lg:border-r lg:border-white/10' : ''}`}
              >
                <div className="font-display text-[56px] font-extrabold text-white leading-none mb-2">
                  {s.num}
                </div>
                <div className="text-sm text-white/65 font-medium">{s.label}</div>
              </div>
            ))}
          </div>
        </FadeIn>
      </div>
    </div>
  );
}
