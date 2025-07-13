import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import Button from "@mui/material/Button";

interface LegacyDataDialogProps {
  open: boolean;
  onClose: () => void;
  onConfirm: () => void;
}

export default function LegacyDataDialog({
  open,
  onClose,
  onConfirm,
}: LegacyDataDialogProps) {
  return (
    <Dialog
      open={open}
      onClose={onClose}
      aria-labelledby="legacy-data-dialog-title"
      aria-describedby="legacy-data-dialog-description"
    >
      <DialogTitle id="legacy-data-dialog-title">
        {"Import Legacy Data?"}
      </DialogTitle>
      <DialogContent>
        <DialogContentText id="legacy-data-dialog-description">
          {
            "This appears to be the first time you have run this application. Do you want to import data from the old software?"
          }
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button onClick={onConfirm} autoFocus>
          Ok
        </Button>
      </DialogActions>
    </Dialog>
  );
}
