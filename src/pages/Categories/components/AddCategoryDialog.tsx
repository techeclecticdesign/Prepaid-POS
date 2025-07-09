import { useState } from "react";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import AppSnackbar from "../../../components/AppSnackbar";
import { useDialogAutofocus } from "../../../hooks/useDialogAutofocus";

interface Props {
  open: boolean;
  onClose: () => void;
  onSubmit: (name: string) => Promise<void>;
  existingNames: string[];
}

export default function AddCategoryDialog({
  open,
  onClose,
  onSubmit,
  existingNames,
}: Props) {
  const [name, setName] = useState("");
  const [error, setError] = useState("");
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const { ref: inputRef, handleDialogEntered } = useDialogAutofocus(open);

  const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const trimmed = name.trim();
    if (!trimmed) return;

    if (existingNames.some((n) => n.toLowerCase() === trimmed.toLowerCase())) {
      setError("That category already exists.");
      setSnackbarOpen(true);
      return;
    }

    try {
      await onSubmit(trimmed);
      setName("");
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setSnackbarOpen(true);
      return;
    }
  };

  return (
    <>
      <Dialog
        open={open}
        onClose={onClose}
        slotProps={{
          transition: {
            onEntered: handleDialogEntered,
          },
        }}
      >
        <form onSubmit={handleFormSubmit}>
          <DialogTitle>Add Category</DialogTitle>
          <DialogContent>
            <TextField
              inputRef={inputRef}
              margin="dense"
              label="Category Name"
              fullWidth
              value={name}
              onChange={(e) => {
                setName(e.target.value);
              }}
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={onClose}>Cancel</Button>
            <Button type="submit" variant="contained">
              Add
            </Button>
          </DialogActions>
        </form>
      </Dialog>
      <AppSnackbar
        open={snackbarOpen}
        message={error}
        onClose={() => setSnackbarOpen(false)}
      />
    </>
  );
}
