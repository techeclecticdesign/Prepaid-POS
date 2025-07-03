import { Link } from "react-router-dom";

export default function AdminSidebar() {
  return (
    <aside className="w-[300px] shrink-0 border-l border-gray-300 p-6 bg-white">
      <h2 className="text-2xl font-semibold mb-4">Admin Pages</h2>
      <nav className="flex flex-col space-y-2">
        <Link to="/operators" className="text-blue-600 hover:underline">
          Operators
        </Link>
        <span className="text-gray-500 cursor-not-allowed">Lorem</span>
        <span className="text-gray-500 cursor-not-allowed">Ipsum</span>
        <span className="text-gray-500 cursor-not-allowed">Dolor</span>
      </nav>
    </aside>
  );
}
