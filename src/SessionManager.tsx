import { useNavigate, Outlet } from "react-router-dom";
import { useEffect } from "react";
import { useAuth } from "./AuthProvider";
import SessionTimeoutDialog from "./components/SessionTimeoutDialog";

export default function SessionManager() {
  const { timedOut, clearTimeoutFlag } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (timedOut) {
      navigate("/", { replace: true });
    }
  }, [timedOut, navigate]);

  return (
    <>
      <Outlet />
      <SessionTimeoutDialog open={timedOut} onClose={clearTimeoutFlag} />
    </>
  );
}
