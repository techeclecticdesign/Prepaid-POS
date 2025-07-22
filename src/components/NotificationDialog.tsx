import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  Typography,
} from "@mui/material";

interface NotificationDialogProps {
  open: boolean;
  onClose: () => void;
  title: string;
  titleColor?: string;
  buttonColor?:
    | "inherit"
    | "primary"
    | "secondary"
    | "error"
    | "info"
    | "success"
    | "warning";
  buttonText?: string;
  children: React.ReactNode;
}

export default function NotificationDialog({
  open,
  onClose,
  title,
  titleColor = "warning.main",
  buttonColor = "warning",
  buttonText = "Ok",
  children,
}: NotificationDialogProps) {
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle sx={{ color: titleColor }}>{title}</DialogTitle>
      <DialogContent>
        <Typography>{children}</Typography>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose} color={buttonColor} variant="contained">
          {buttonText}
        </Button>
      </DialogActions>
    </Dialog>
  );
}
