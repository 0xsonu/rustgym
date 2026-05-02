import { motion } from 'framer-motion';
import Navbar from '@/components/layout/Navbar';
import Footer from '@/components/layout/Footer';
import HeroSection from './landing/HeroSection';
import AboutSection from './landing/AboutSection';
import FeaturesSection from './landing/FeaturesSection';
import TrainingFormatSection from './landing/TrainingFormatSection';
import QuestMapSection from './landing/QuestMapSection';
import ProspectsSection from './landing/ProspectsSection';
import StatsBanner from './landing/StatsBanner';
import ReviewsSection from './landing/ReviewsSection';
import MediaSection from './landing/MediaSection';
import EduSection from './landing/EduSection';
import FinalCTA from './landing/FinalCTA';

function Divider() {
  return <hr className="border-t border-border m-0" />;
}

const fadeInVariants = {
  hidden: { opacity: 0, y: 24 },
  visible: { opacity: 1, y: 0, transition: { duration: 0.7, ease: 'easeOut' as const } },
};

export function FadeIn({ children, className }: { children: React.ReactNode; className?: string }) {
  return (
    <motion.div
      variants={fadeInVariants}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, amount: 0.1 }}
      className={className}
    >
      {children}
    </motion.div>
  );
}

export default function Landing() {
  return (
    <div className="min-h-screen bg-dark-950 text-text-primary font-body">
      <Navbar />
      <HeroSection />
      <Divider />
      <AboutSection />
      <Divider />
      <FeaturesSection />
      <Divider />
      <TrainingFormatSection />
      <Divider />
      <QuestMapSection />
      <Divider />
      <ProspectsSection />
      <StatsBanner />
      <Divider />
      <ReviewsSection />
      <Divider />
      <MediaSection />
      <Divider />
      <EduSection />
      <Divider />
      <FinalCTA />
      <Footer />
    </div>
  );
}
