import AdminSidebar from "./components/AdminSidebar";

export default function AdminPage() {
  return (
    <main className="min-h-screen flex">
      {/* Left side: content area */}
      <div className="flex-1 flex items-center justify-center bg-gray-50">
        <img src="mock.png" />
      </div>

      {/* Right side: sidebar */}
      <AdminSidebar />
    </main>
  );
}
