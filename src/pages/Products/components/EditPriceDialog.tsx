import { useState, useEffect, useRef } from "react";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

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
      setSnackbarOpen(false); // Close any open snackbar when dialog opens
      if (inputRef.current) {
        inputRef.current.focus(); // autofocus on textfield when modal opens
      }
    } else if (document.activeElement instanceof HTMLElement) {
      // Remove focus from button
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
      setNewPrice(0); // Set to invalid value to prevent saving
      return;
    }

    const parsedValue = Number.parseFloat(value);

    if (isNaN(parsedValue) || parsedValue <= 0) {
      setSnackbarMessage("Price must be a positive, non-zero number.");
      setSnackbarSeverity("error");
      setSnackbarOpen(true);
      setNewPrice(0); // Set to invalid value to prevent saving
      return;
    }

    // If input is valid, close snackbar and update price
    setSnackbarOpen(false);
    setNewPrice(Math.round(parsedValue * 100));
  };

  const handleSave = async () => {
    // Perform final validation before saving
    const parsedValue = Number.parseFloat(inputValue);
    if (isNaN(parsedValue) || parsedValue <= 0 || inputValue.startsWith("$")) {
      // Snackbar message would already be set by handleInputChange, just prevent save
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
        <p className="text-sm text-gray-700">
          Any changes to prices are logged and reported. If you wish to
          continue, please enter the new price below.
        </p>
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
      <Snackbar
        open={snackbarOpen}
        autoHideDuration={6000}
        onClose={handleCloseSnackbar}
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      >
        <Alert
          onClose={handleCloseSnackbar}
          severity={snackbarSeverity}
          sx={{ width: "100%" }}
        >
          {snackbarMessage}
        </Alert>
      </Snackbar>
    </Dialog>
  );
}
