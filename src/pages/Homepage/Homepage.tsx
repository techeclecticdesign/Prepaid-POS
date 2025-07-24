import { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import BarcodeScanner from "../../lib/barcode";
import AppButton from "../../components/AppButton";
import StaffLoginDialog from "./components/StaffLoginDialog";
import LegacyDataDialog from "./components/LegacyDataDialog";
import { useAuth } from "../../AuthProvider";
import useOperators from "../../hooks/useOperators";
import { useLegacyDataCheck } from "./hooks/useLegacyDataCheck";
import useLegacyDataActions from "./hooks/useLegacyDataActions";
import type Operator from "../../models/Operator";

export default function App() {
  const { operators, isLoading: isLoadingOperators, refresh } = useOperators();
  const { importLegacyData } = useLegacyDataActions();
  const { shouldPromptForLegacyData, acknowledgePrompt } = useLegacyDataCheck(
    operators,
    isLoadingOperators,
  );

  const operatorsRef = useRef<Operator[]>(operators);
  const errorTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const { loggedIn, login, logout, setActiveOperator, passwordRequired } =
    useAuth();
  const [showLogin, setShowLogin] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);
  const navigate = useNavigate();

  const [showLegacyDataDialog, setShowLegacyDataDialog] = useState(false);
  const saved = localStorage.getItem("barcode");

  function isValidScannerType(val: string | null): val is "Zebra" | "Generic" {
    return val === "Zebra" || val === "Generic";
  }

  let scannerType: "Zebra" | "Generic";
  if (isValidScannerType(saved)) {
    scannerType = saved;
  } else {
    localStorage.setItem("barcode", "Zebra");
    scannerType = "Zebra";
  }

  // show the dialog when the hook indicates it should
  useEffect(() => {
    if (shouldPromptForLegacyData) {
      setShowLegacyDataDialog(true);
    }
  }, [shouldPromptForLegacyData]);

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
    const scanNum = Number.parseInt(scan, 10);
    const matched = operatorsRef.current.find((o) => o.mdoc === scanNum);
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
    const prefix = scannerType === "Zebra" ? "~" : "";

    const scanner = new BarcodeScanner({
      prefix,
      suffix: "",
      timeout: 50,
      shouldCapture: () => true,
      barcodeCallback: handleScan,
    });

    return () => {
      scanner.destroy(); // removes keydown listener
    };
  }, [scannerType]);

  const handleLegacyDataOk = async () => {
    await importLegacyData();
    setShowLegacyDataDialog(false);
    acknowledgePrompt();
    refresh();
  };

  const handleLegacyDataCancel = () => {
    console.log("User cancelled legacy data import.");
    setShowLegacyDataDialog(false);
    acknowledgePrompt();
  };

  return (
    <Box className="min-h-screen flex flex-col items-center justify-start">
      <Box className="h-20">
        {scanError && (
          <Typography
            variant="h5"
            component="h1"
            className="font-bold text-center mt-4"
            sx={{ color: "error.main" }}
          >
            {scanError}
          </Typography>
        )}
      </Box>
      <Typography
        variant="h4"
        component="h1"
        className="font-bold text-center"
        sx={{ color: "text.primary" }}
      >
        Click your name or scan your ID to get started.
      </Typography>
      <Box className="flex flex-col gap-4 mt-20">
        {/* Display operator buttons only if operators are loaded and not empty */}
        {!isLoadingOperators && currentOperators.length > 0
          ? currentOperators.map((o) => (
              <AppButton
                key={o.mdoc}
                text={o.name}
                variant="outlined"
                sx={{ width: "14rem" }}
                onClick={() => {
                  setActiveOperator(o);
                  navigate("/sales");
                }}
              />
            ))
          : null}
        {/* If no password is set, skip auth entirely */}
        {!passwordRequired ? (
          <AppButton
            onClick={async () => {
              // ensure loggedIn is true so /admin route exists
              await login("");
              navigate("/admin");
            }}
            text="Admin Controls"
            sx={{ width: "14rem" }}
          />
        ) : !loggedIn ? (
          <AppButton
            onClick={() => setShowLogin(true)}
            text="Admin Login"
            sx={{ width: "14rem" }}
          />
        ) : (
          <AppButton onClick={logout} text="Log Out" />
        )}
      </Box>
      {/* Only show login dialog if password is actually required */}
      {passwordRequired && (
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
      )}

      <LegacyDataDialog
        open={showLegacyDataDialog}
        onClose={handleLegacyDataCancel}
        onConfirm={handleLegacyDataOk}
      />
    </Box>
  );
}
