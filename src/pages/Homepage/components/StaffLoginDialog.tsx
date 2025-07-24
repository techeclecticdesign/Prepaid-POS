import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Box from "@mui/material/Box";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
import CircularProgress from "@mui/material/CircularProgress";
import AppButton from "../../../components/AppButton";
import AppSnackbar from "../../../components/AppSnackbar";

interface Props {
  open: boolean;
  onClose: () => void;
  onLoginSuccess: (password: string) => void;
}

export default function StaffLoginDialog({
  open,
  onClose,
  onLoginSuccess,
}: Props) {
  const [pw, setPw] = useState("");
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitting(true);
    try {
      await invoke("staff_login", { password: pw });
      onLoginSuccess(pw);
      return;
    } catch {
      onClose();
      setSnackbarOpen(true);
      setSubmitting(false);
    }
    setPw("");
  };

  return (
    <>
      <Dialog open={open} onClose={onClose} disableRestoreFocus>
        <form onSubmit={handleSubmit}>
          <DialogTitle>Staff Login</DialogTitle>
          <DialogContent>
            <TextField
              margin="dense"
              type="password"
              label="Password"
              value={pw}
              onChange={(e) => setPw(e.target.value)}
              autoFocus
              fullWidth
            />
          </DialogContent>
          <DialogActions>
            <Box sx={{ mx: "auto" }}>
              {submitting ? (
                <CircularProgress size={28} />
              ) : (
                <AppButton type="submit" text="Submit" sx={{ minWidth: 200 }} />
              )}
            </Box>
          </DialogActions>
        </form>
      </Dialog>
      <AppSnackbar
        open={snackbarOpen}
        message="Incorrect password"
        onClose={() => setSnackbarOpen(false)}
      />
    </>
  );
}
