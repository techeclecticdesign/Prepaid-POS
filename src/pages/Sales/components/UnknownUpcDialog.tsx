import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  Typography,
} from "@mui/material";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function UnknownUpcDialog({ open, onClose }: Props) {
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle sx={{ color: "warning.main" }}>
        Unrecognized Product
      </DialogTitle>
      <DialogContent>
        <Typography>
          Scanned product does not have a recognized UPC. Item cannot be sold
          until it has been registered in the Products page.
        </Typography>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose} color="warning" variant="contained">
          Ok
        </Button>
      </DialogActions>
    </Dialog>
  );
}
