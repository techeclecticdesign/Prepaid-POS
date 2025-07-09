import { Navigate, Outlet, useLocation } from "react-router-dom";
import { useAuth } from "../AuthProvider";

// redirects to homepage if not logged in
export default function ProtectedLayout() {
  const { loggedIn } = useAuth();
  const location = useLocation();

  if (!loggedIn) {
    return <Navigate to="/" state={{ from: location }} replace />;
  }

  return <Outlet />;
}
