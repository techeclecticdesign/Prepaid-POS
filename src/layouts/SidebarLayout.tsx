import { Outlet } from "react-router-dom";

interface SidebarLayoutProps {
  Sidebar: () => JSX.Element;
}

export default function SidebarLayout({ Sidebar }: SidebarLayoutProps) {
  return (
    <main className="h-screen flex">
      <div className="flex-1 flex items-center justify-center p-6">
        <Outlet />
      </div>

      {<Sidebar />}
    </main>
  );
}
