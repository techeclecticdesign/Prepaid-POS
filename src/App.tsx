import { useState } from "react";
import AppButton from "./components/AppButton";
import StaffLoginDialog from "./components/StaffLoginDialog";
import { useAuth } from "./AuthProvider";
import { useOperators } from "./hooks/useOperators";

export default function App() {
  const operators = useOperators();
  const { loggedIn, login, logout } = useAuth();
  const [showLogin, setShowLogin] = useState(false);

  return (
    <main className="min-h-screen flex flex-col items-center justify-start py-10">
      <h1 className="text-4xl font-bold my-20 text-center">
        Click your name or scan your ID to get started.
      </h1>
      <div>
        {operators.map((o) => (
          <AppButton key={o.id} text={o.name} />
        ))}

        {!loggedIn ? (
          <AppButton
            variant="contained"
            onClick={() => setShowLogin(true)}
            text="Admin Login"
          />
        ) : (
          <AppButton variant="contained" onClick={logout} text="Log Out" />
        )}
      </div>

      <StaffLoginDialog
        open={showLogin}
        onClose={() => setShowLogin(false)}
        onLoginSuccess={async (pw) => {
          const ok = await login(pw);
          if (ok) setShowLogin(false);
        }}
      />
    </main>
  );
}
