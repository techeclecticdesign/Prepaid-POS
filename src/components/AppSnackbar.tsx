import { Snackbar, Alert } from "@mui/material";

interface Props {
  open: boolean;
  message: string;
  onClose: () => void;
  severity?: "error" | "warning" | "info" | "success";
}

export default function AppSnackbar({
  open,
  message,
  onClose,
  severity = "error",
}: Props) {
  return (
    <Snackbar
      open={open}
      autoHideDuration={4000}
      anchorOrigin={{ vertical: "top", horizontal: "center" }}
      onClose={onClose}
    >
      <Alert severity={severity} onClose={onClose} sx={{ width: "100%" }}>
        {message}
      </Alert>
    </Snackbar>
  );
}
