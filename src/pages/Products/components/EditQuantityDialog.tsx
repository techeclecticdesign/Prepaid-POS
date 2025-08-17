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
  initialQty: number;
  onClose: () => void;
  onSubmit: (oldQty: number, newQty: number, reason: string) => Promise<void>;
}

export default function EditQuantityDialog({
  open,
  initialQty,
  onClose,
  onSubmit,
}: Props) {
  const [newQty, setNewQty] = useState(initialQty);
  const [inputValue, setInputValue] = useState(String(initialQty));
  const [reason, setReason] = useState("");
  const [reasonInput, setReasonInput] = useState("");
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [snackbarSeverity, setSnackbarSeverity] = useState<
    "error" | "warning" | "info" | "success"
  >("error");
  const inputRef = useRef<HTMLInputElement>(null);

  // Reset whenever dialog opens
  useEffect(() => {
    if (open) {
      setNewQty(initialQty);
      setInputValue(String(initialQty));
      setReason("");
      setReasonInput("");
      setSnackbarOpen(false);
      inputRef.current?.focus();
    } else if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }, [open, initialQty]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e.target.value;
    setInputValue(val);

    // must be integer >= 0
    const parsed = Number.parseInt(val, 10);
    if (Number.isNaN(parsed) || parsed < 0) {
      setSnackbarMessage("Quantity must be a non-negative integer.");
      setSnackbarSeverity("error");
      setSnackbarOpen(true);
      setNewQty(0);
    } else {
      setSnackbarOpen(false);
      setNewQty(parsed);
    }
  };

  const handleReasonChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setReasonInput(e.target.value);
    setReason(e.target.value);
  };

  const handleSave = async () => {
    // block invalid
    const quantityChange = newQty - initialQty;

    if (quantityChange < 0 && !reason.trim()) {
      setSnackbarMessage("Reason is required for negative quantity changes.");
      setSnackbarSeverity("error");
      setSnackbarOpen(true);
      return;
    }

    if (snackbarOpen) return; // Block if quantity input is invalid
    await onSubmit(initialQty, newQty, reason.trim());
    onClose();
  };

  const handleCloseSnackbar = (
    _event?: React.SyntheticEvent | Event,
    reason?: string,
  ) => {
    if (reason === "clickaway") return;
    setSnackbarOpen(false);
  };

  // autofocus after transition
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
      <DialogTitle>Adjust Quantity</DialogTitle>
      <DialogContent dividers sx={{ gap: 4 }}>
        <Typography variant="body2" sx={{ color: "text.secondary", mb: 2 }}>
          Any changes to inventory are logged and reported. If you wish to
          continue, please enter the new quantity below.
        </Typography>
        <TextField
          label="New Quantity"
          value={inputValue}
          onChange={handleInputChange}
          fullWidth
          error={snackbarOpen && snackbarSeverity === "error"}
          inputRef={inputRef}
          sx={{ mb: 2 }}
        />
        <TextField
          label="Reason"
          value={reasonInput}
          onChange={handleReasonChange}
          fullWidth
          error={
            newQty - initialQty < 0 &&
            !reason.trim() &&
            snackbarOpen &&
            snackbarSeverity === "error"
          }
          helperText={
            newQty - initialQty < 0 &&
            !reason.trim() &&
            snackbarOpen &&
            snackbarSeverity === "error"
              ? "Reason is required for negative changes"
              : ""
          }
          placeholder="Reason for adjustment"
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
