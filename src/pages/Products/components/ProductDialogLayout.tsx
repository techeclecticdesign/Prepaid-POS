import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Button from "@mui/material/Button";
import Box from "@mui/material/Box";

interface Props {
  open: boolean;
  title: string;
  onClose: () => void;
  children: React.ReactNode;
  onSubmit: () => void;
  submitText?: string;
  leftActions?: React.ReactNode;
}

export default function ProductDialogLayout({
  open,
  title,
  onClose,
  children,
  onSubmit,
  submitText = "Save",
  leftActions,
}: Props) {
  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
      <DialogTitle>{title}</DialogTitle>
      <DialogContent dividers>{children}</DialogContent>
      <DialogActions
        sx={
          leftActions
            ? { justifyContent: "space-between" }
            : { justifyContent: "flex-end" }
        }
      >
        {leftActions && leftActions}
        {/* Only render leftActions if it's not null/undefined */}
        <Box sx={{ display: "flex", gap: 2 }}>
          <Button onClick={onClose}>Cancel</Button>
          <Button variant="contained" onClick={onSubmit}>
            {submitText}
          </Button>
        </Box>
      </DialogActions>
    </Dialog>
  );
}
