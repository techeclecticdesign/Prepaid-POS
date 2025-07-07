import { Link, useNavigate } from "react-router-dom";

export default function AdminSidebar() {
  const navigate = useNavigate();
  return (
    <aside className="sticky top-0 self-start w-xs shrink-0 border-l border-gray-300 p-6 bg-white h-screen">
      <h2 className="text-2xl font-semibold mb-4">Pages</h2>
      <nav className="flex flex-col space-y-2">
        <Link to="/products" className="text-blue-600 hover:underline">
          Products
        </Link>
        <span className="text-gray-500 cursor-not-allowed">Lorem</span>
        <span className="text-gray-500 cursor-not-allowed">Ipsum</span>
        <span className="text-gray-500 cursor-not-allowed">Dolor</span>
      </nav>
      <button
        onClick={() => {
          navigate("/");
        }}
        className="mt-6 text-red-600 hover:underline text-left"
      >
        Sign Out
      </button>
    </aside>
  );
}
