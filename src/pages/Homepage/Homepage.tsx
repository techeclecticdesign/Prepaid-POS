import { useState, useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";
import BarcodeScanner from "../../lib/barcode";
import AppButton from "../../components/AppButton";
import StaffLoginDialog from "./components/StaffLoginDialog";
import { useAuth } from "../../AuthProvider";
import useOperators from "../../hooks/useOperators";

export default function App() {
  const { operators } = useOperators();
  const operatorsRef = useRef(operators);
  const errorTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const { loggedIn, login, logout } = useAuth();
  const [showLogin, setShowLogin] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);
  const navigate = useNavigate();
  const { setActiveOperator } = useAuth();

  const currentOperators = operators.filter(
    (o) => o.stop === null || new Date(o.start) > new Date(o.stop),
  );

  useEffect(() => {
    operatorsRef.current = operators;
  }, [operators]);

  const handleScan = (scan: string) => {
    if (!/^\d+$/.test(scan)) {
      return;
    }
    const scanNum = parseInt(scan, 10);
    const matched = operatorsRef.current.find((o) => o.id === scanNum);
    if (!matched) {
      setScanError("Scan input does not match any operator MDOC.");
      if (errorTimerRef.current) {
        clearTimeout(errorTimerRef.current);
      }
      errorTimerRef.current = setTimeout(() => setScanError(null), 5000);
      return;
    }
    setScanError(null);
    if (errorTimerRef.current) {
      clearTimeout(errorTimerRef.current);
    }
    setActiveOperator(matched);
    navigate("/sales");
  };

  useEffect(() => {
    new BarcodeScanner({
      timeout: 50,
      shouldCapture: () => true,
      barcodeCallback: handleScan,
    });
  }, []);

  useEffect(() => {
    operatorsRef.current = operators;
  }, [operators]);

  return (
    <div className="min-h-screen flex flex-col items-center justify-start">
      <div className="h-20">
        {scanError && (
          <h1 className="text-3xl font-bold text-center mt-4 text-red-500">
            {scanError}
          </h1>
        )}
      </div>
      <h1 className="text-4xl font-bold mb-20 text-center">
        Click your name or scan your ID to get started.
      </h1>
      <div className="flex flex-col gap-4">
        {currentOperators.map((o) => (
          <AppButton
            key={o.id}
            text={o.name}
            variant="outlined"
            sx={{ width: 250 }}
            onClick={() => {
              setActiveOperator(o);
              navigate("/sales");
            }}
          />
        ))}

        {!loggedIn ? (
          <AppButton
            onClick={() => setShowLogin(true)}
            text="Admin Login"
            sx={{ width: 250 }}
          />
        ) : (
          <AppButton onClick={logout} text="Log Out" />
        )}
      </div>

      <StaffLoginDialog
        open={showLogin}
        onClose={() => setShowLogin(false)}
        onLoginSuccess={async (pw) => {
          const ok = await login(pw);
          if (ok) {
            setShowLogin(false);
            navigate("/admin");
          }
        }}
      />
    </div>
  );
}
