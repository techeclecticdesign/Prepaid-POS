import { useState, useEffect, useRef } from "react";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import AppSnackbar from "../../../components/AppSnackbar";

interface Props {
  open: boolean;
  initialPriceDollars: number;
  onClose: () => void;
  onSubmit: (oldPriceCents: number, newPriceCents: number) => Promise<void>;
}

export default function EditPriceDialog({
  open,
  initialPriceDollars,
  onClose,
  onSubmit,
}: Props) {
  const [newPrice, setNewPrice] = useState(initialPriceDollars);
  const [inputValue, setInputValue] = useState(
    (initialPriceDollars / 100).toFixed(2),
  );
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [snackbarSeverity, setSnackbarSeverity] = useState<
    "error" | "warning" | "info" | "success"
  >("error");
  const inputRef = useRef<HTMLInputElement>(null);

  // Reset input whenever dialog opens
  useEffect(() => {
    if (open) {
      setNewPrice(initialPriceDollars);
      setInputValue((initialPriceDollars / 100).toFixed(2));
      setSnackbarOpen(false);
      if (inputRef.current) {
        inputRef.current.focus(); // autofocus on textfield when modal opens
      }
    } else if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }, [open, initialPriceDollars]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setInputValue(value);
    if (value.startsWith("$")) {
      setSnackbarMessage("Please omit $ sign from price.");
      setSnackbarSeverity("error");
      setSnackbarOpen(true);
      setNewPrice(0);
      return;
    }
    const parsedValue = Number.parseFloat(value);
    if (Number.isNaN(parsedValue) || parsedValue <= 0) {
      setSnackbarMessage("Price must be a positive, non-zero number.");
      setSnackbarSeverity("error");
      setSnackbarOpen(true);
      setNewPrice(0);
      return;
    }
    setSnackbarOpen(false);
    setNewPrice(Math.round(parsedValue * 100));
  };

  const handleSave = async () => {
    const parsedValue = Number.parseFloat(inputValue);
    if (
      Number.isNaN(parsedValue) ||
      parsedValue <= 0 ||
      inputValue.startsWith("$")
    ) {
      // prevent save
      return;
    }
    await onSubmit(initialPriceDollars, newPrice);
    onClose();
  };

  const handleCloseSnackbar = (
    _event?: React.SyntheticEvent | Event,
    reason?: string,
  ) => {
    if (reason === "clickaway") {
      return;
    }
    setSnackbarOpen(false);
  };

  // Autofocus on modal open
  const handleDialogEntered = () => {
    inputRef.current?.focus();
  };

  return (
    <Dialog
      open={open}
      onClose={onClose}
      fullWidth
      maxWidth="xs"
      onKeyDown={(e) => {
        if (e.key === "Enter") handleSave();
      }}
      slotProps={{
        transition: {
          onEntered: handleDialogEntered,
        },
      }}
    >
      <DialogTitle>Adjust Price</DialogTitle>
      <DialogContent dividers className="space-y-4">
        <Typography variant="body2" sx={{ color: "text.secondary", mb: 2 }}>
          Any changes to prices are logged and reported. If you wish to
          continue, please enter the new price below.
        </Typography>
        <TextField
          label="New Price ($)"
          value={inputValue}
          onChange={handleInputChange}
          fullWidth
          error={snackbarOpen && snackbarSeverity === "error"}
          inputRef={inputRef}
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button variant="contained" onClick={handleSave}>
          Save
        </Button>
      </DialogActions>
      <AppSnackbar
        open={snackbarOpen}
        message={snackbarMessage}
        onClose={handleCloseSnackbar}
        severity={snackbarSeverity}
      />
    </Dialog>
  );
}
