import { Routes, Route } from "react-router-dom";
import HomePage from "./pages/Homepage/Homepage";
import AdminPage from "./pages/AdminPage/AdminPage";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/admin" element={<AdminPage />} />
    </Routes>
  );
}
