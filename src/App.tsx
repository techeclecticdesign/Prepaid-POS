import SessionManager from "./SessionManager";
import ProtectedLayout from "./layouts/ProtectedLayout";
import AdminLayout from "./layouts/AdminLayout";
import HomePage from "./pages/Homepage/Homepage";
import AdminPage from "./pages/AdminPage/AdminPage";
import Operators from "./pages/Operators/Operators";
import { useHandleActivity } from "./hooks/useHandleActivity";
import { useSessionPolling } from "./hooks/useSessionPolling";
import { Routes, Route } from "react-router-dom";

export default function App() {
  useHandleActivity();
  useSessionPolling();

  return (
    <Routes>
      <Route element={<SessionManager />}>
        <Route path="/" element={<HomePage />} />

        <Route element={<ProtectedLayout />}>
          <Route element={<AdminLayout />}>
            <Route path="/admin" element={<AdminPage />} />
            <Route path="/operators" element={<Operators />} />
          </Route>
        </Route>
      </Route>
    </Routes>
  );
}
