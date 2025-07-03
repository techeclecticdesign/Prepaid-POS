import SessionManager from "./SessionManager";
import HomePage from "./pages/Homepage/Homepage";
import AdminPage from "./pages/AdminPage/AdminPage";
import Operators from "./pages/Operators/Operators";
import ProtectedLayout from "./ProtectedLayout";
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
          <Route path="/admin" element={<AdminPage />} />
          <Route path="/operators" element={<Operators />} />
        </Route>
      </Route>
    </Routes>
  );
}
