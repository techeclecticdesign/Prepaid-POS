import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import AppButton from "../../components/AppButton";
import AppSnackbar from "../../components/AppSnackbar";
import { useAuth } from "../../AuthProvider";

export default function ChangePasswordPage() {
  const { passwordRequired, refreshPasswordRequired } = useAuth();
  const [oldPw, setOldPw] = useState("");
  const [newPw, setNewPw] = useState("");
  const [confirmPw, setConfirmPw] = useState("");
  const [snack, setSnack] = useState<{
    open: boolean;
    message: string;
    success: boolean;
  }>({
    open: false,
    message: "",
    success: false,
  });

  const clearAll = () => {
    setOldPw("");
    setNewPw("");
    setConfirmPw("");
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (newPw !== confirmPw) {
      setSnack({
        open: true,
        message: "New passwords don't match",
        success: false,
      });
      clearAll();
      return;
    }
    try {
      await invoke("change_password", {
        oldPassword: oldPw,
        newPassword: newPw,
      });
      await refreshPasswordRequired();
      setSnack({ open: true, message: "Password changed!", success: true });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      setSnack({ open: true, message: msg, success: false });
    } finally {
      clearAll();
    }
  };

  const handleDelete = async () => {
    try {
      await invoke("delete_password");
      await refreshPasswordRequired();
      setSnack({
        open: true,
        message: "Password deleted!",
        success: true,
      });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      setSnack({ open: true, message: msg, success: false });
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
        p: 8,
      }}
    >
      <Box
        sx={{
          maxWidth: 400,
          mx: "auto",
        }}
      >
        <Typography
          variant="h4"
          component="h1"
          sx={{
            fontWeight: "bold",
            textAlign: "center",
            color: "text.primary",
          }}
        >
          Change Password
        </Typography>

        <Box
          component="form"
          onSubmit={handleSubmit}
          sx={{
            display: "flex",
            flexDirection: "column",
            gap: 3,
            mt: 8,
          }}
        >
          {passwordRequired && (
            <TextField
              label="Old Password"
              type="password"
              fullWidth
              value={oldPw}
              onChange={(e) => setOldPw(e.target.value)}
            />
          )}
          <TextField
            label="New Password"
            type="password"
            fullWidth
            value={newPw}
            onChange={(e) => setNewPw(e.target.value)}
          />
          <TextField
            label="Confirm New Password"
            type="password"
            fullWidth
            value={confirmPw}
            onChange={(e) => setConfirmPw(e.target.value)}
          />

          <Box sx={{ display: "flex", textAlign: "center", gap: 2, mt: 1 }}>
            <AppButton
              type="submit"
              variant="contained"
              text="Change Password"
            />
            <AppButton
              variant="contained"
              color="error"
              text="Delete Password"
              onClick={handleDelete}
            />
          </Box>
        </Box>

        <AppSnackbar
          open={snack.open}
          message={snack.message}
          onClose={() => setSnack((s) => ({ ...s, open: false }))}
          severity={snack.success ? "success" : "error"}
        />
      </Box>
    </Box>
  );
}
