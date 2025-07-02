import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from "@mui/material";
import { Button } from "@mui/material";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SessionTimeoutDialog({ open, onClose }: Props) {
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>Session Timed Out</DialogTitle>
      <DialogContent>You have been logged out due to inactivity.</DialogContent>
      <DialogActions>
        <Button variant="contained" onClick={onClose}>
          Ok
        </Button>
      </DialogActions>
    </Dialog>
  );
}
