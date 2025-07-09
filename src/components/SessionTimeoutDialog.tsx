import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import AppButton from "./AppButton";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SessionTimeoutDialog({ open, onClose }: Props) {
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>Session Timed Out</DialogTitle>
      <DialogContent>You have been logged out due to inactivity.</DialogContent>
      <DialogActions sx={{ justifyContent: "center" }}>
        <AppButton onClick={onClose} text="Ok" />
      </DialogActions>
    </Dialog>
  );
}
