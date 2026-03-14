import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import LandingPage from '../pages/LandingPage';
import StartupIntakePage from '../pages/StartupIntakePage';
import WorkspaceHomePage from '../pages/WorkspaceHomePage';
import DashboardPage from '../pages/internal/DashboardPage';

function AppRouter() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/cn" element={<LandingPage />} />
        <Route path="/cn/*" element={<LandingPage />} />
        <Route path="/start" element={<StartupIntakePage />} />
        <Route path="/workspace" element={<WorkspaceHomePage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}

export default AppRouter;
