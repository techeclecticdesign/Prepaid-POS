import { Outlet } from "react-router-dom";
import AdminSidebar from "../components/AdminSidebar";

export default function AdminLayout() {
  return (
    <main className="min-h-screen flex">
      <div className="flex-1 flex items-center justify-center p-6">
        <Outlet />
      </div>

      <AdminSidebar />
    </main>
  );
}
