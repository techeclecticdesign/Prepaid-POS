import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import AppButton from "../../components/AppButton";
import AppSnackbar from "../../components/AppSnackbar";

export default function WeeklyLimitPage() {
  // State for the current & edited limit (in dollars)
  const [limit, setLimit] = useState<string>("");
  const [originalLimit, setOriginalLimit] = useState<string>("");
  const [snack, setSnack] = useState<{
    open: boolean;
    message: string;
    severity: "success" | "error" | "info" | "warning";
  }>({ open: false, message: "", severity: "info" });

  // Load the current limit on mount
  useEffect(() => {
    (async () => {
      try {
        const val: number = await invoke("get_weekly_limit"); // returns cents
        const dollars = (val / 100).toFixed(2);
        setLimit(dollars);
        setOriginalLimit(dollars);
      } catch (err) {
        setSnack({
          open: true,
          message: err instanceof Error ? err.message : String(err),
          severity: "error",
        });
      }
    })();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    // unchanged?
    if (limit === originalLimit) {
      setSnack({
        open: true,
        message: "No submission was required for unchanged limit",
        severity: "info",
      });
      return;
    }
    // validate: must be a positive dollar amount
    const parsed = Number(limit);
    if (isNaN(parsed) || parsed < 0) {
      setSnack({
        open: true,
        message: "Limit must be a non-negative number, e.g. 123.45",
        severity: "warning",
      });
      return;
    }
    // convert to cents
    const cents = Math.round(parsed * 100);
    try {
      await invoke("set_weekly_limit", { limit: cents });
      setOriginalLimit(limit);
      setSnack({
        open: true,
        message: "Submission was successful.",
        severity: "success",
      });
    } catch (err) {
      setSnack({
        open: true,
        message: err instanceof Error ? err.message : String(err),
        severity: "error",
      });
    }
  };

  return (
    <Box
      component="main"
      sx={{
        position: "fixed",
        top: 0,
        width: "calc(100vw - 21.5rem)",
        height: "100vh",
        overflow: "auto",
        p: 6,
      }}
    >
      <Box sx={{ maxWidth: 400, mx: "auto" }}>
        <Typography
          variant="h4"
          component="h1"
          sx={{
            fontWeight: "bold",
            textAlign: "center",
            color: "text.primary",
          }}
        >
          Weekly Spending Limit
        </Typography>

        <Box
          component="form"
          onSubmit={handleSubmit}
          sx={{ display: "flex", flexDirection: "column", gap: 3, mt: 4 }}
        >
          <TextField
            label="Weekly Limit (USD)"
            type="text"
            fullWidth
            value={limit}
            onChange={(e) => setLimit(e.target.value)}
            helperText="Enter amount in dollars, e.g. 100.00"
          />

          <Box
            sx={{ display: "flex", justifyContent: "center", gap: 2, mt: 2 }}
          >
            <AppButton type="submit" variant="contained" text="Save Limit" />
          </Box>
        </Box>

        <AppSnackbar
          open={snack.open}
          message={snack.message}
          severity={snack.severity}
          onClose={() => setSnack((s) => ({ ...s, open: false }))}
        />
      </Box>
    </Box>
  );
}
