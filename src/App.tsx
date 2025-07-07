import SessionManager from "./SessionManager";
import ProtectedLayout from "./layouts/ProtectedLayout";
import SidebarLayout from "./layouts/SidebarLayout";
import AdminSidebar from "./components/AdminSidebar";
import OperatorSidebar from "./components/OperatorSidebar";
import HomePage from "./pages/Homepage/Homepage";
import Sales from "./pages/Sales/Sales";
import Products from "./pages/Products/Products";
import AdminPage from "./pages/AdminPage/AdminPage";
import Operators from "./pages/Operators/Operators";
import Categories from "./pages/Categories/Categories";
import { useHandleActivity } from "./hooks/useHandleActivity";
import { useSessionPolling } from "./hooks/useSessionPolling";
import { useDisableContextMenu } from "./hooks/useDisableContextMenu";
import { Routes, Route } from "react-router-dom";

export default function App() {
  useHandleActivity();
  useSessionPolling();
  useDisableContextMenu();

  return (
    <Routes>
      <Route element={<SessionManager />}>
        <Route path="/" element={<HomePage />} />

        <Route element={<SidebarLayout Sidebar={OperatorSidebar} />}>
          <Route path="/sales" element={<Sales />} />
          <Route path="/products" element={<Products />} />
        </Route>

        <Route element={<ProtectedLayout />}>
          <Route element={<SidebarLayout Sidebar={AdminSidebar} />}>
            <Route path="/admin" element={<AdminPage />} />
            <Route path="/operators" element={<Operators />} />
            <Route path="/categories" element={<Categories />} />
          </Route>
        </Route>
      </Route>
    </Routes>
  );
}
