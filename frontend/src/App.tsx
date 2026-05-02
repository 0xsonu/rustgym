import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Landing from '@/pages/Landing';
import Login from '@/pages/Login';
import Register from '@/pages/Register';
import Quests from '@/pages/Quests';
import QuestDetail from '@/pages/QuestDetail';
import LevelDetail from '@/pages/LevelDetail';
import TaskSolver from '@/pages/TaskSolver';
import Dashboard from '@/pages/Dashboard';
import Leaderboard from '@/pages/Leaderboard';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30_000,
      retry: 1,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Landing />} />
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
          <Route path="/quests" element={<Quests />} />
          <Route path="/quests/:slug" element={<QuestDetail />} />
          <Route path="/quests/:questSlug/levels/:levelSlug" element={<LevelDetail />} />
          <Route path="/tasks/:slug" element={<TaskSolver />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/leaderboard" element={<Leaderboard />} />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}

export default App;
