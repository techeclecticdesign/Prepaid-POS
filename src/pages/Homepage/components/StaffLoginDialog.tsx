import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
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

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke("staff_login", { password: pw });
      onLoginSuccess(pw);
      setPw("");
    } catch {
      setSnackbarOpen(true);
    }
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
            <AppButton type="submit" text="Submit" sx={{ width: "100%" }} />
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
