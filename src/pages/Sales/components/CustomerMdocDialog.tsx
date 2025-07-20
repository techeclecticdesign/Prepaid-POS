import { useState } from "react";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

interface Props {
  open: boolean;
  onClose: () => void;
  onSubmitMdoc: (mdoc: number) => void;
}

export default function CustomerMdocDialog({
  open,
  onClose,
  onSubmitMdoc,
}: Props) {
  const [mdocInput, setMdocInput] = useState("");

  const handleSubmit = (event?: React.MouseEvent) => {
    if (event) {
      event.stopPropagation();
    }
    const mdoc = Number.parseInt(mdocInput, 10);
    if (!Number.isNaN(mdoc) && mdoc > 0) {
      onSubmitMdoc(mdoc);
      setMdocInput("");
      onClose();
    }
  };

  const handleClose = (event?: React.MouseEvent) => {
    if (event) {
      event.stopPropagation();
    }
    setMdocInput("");
    onClose();
  };

  const handleDialogClick = (event: React.MouseEvent) => {
    event.stopPropagation();
  };

  return (
    <Dialog
      open={open}
      onClose={() => handleClose()}
      maxWidth="xs"
      fullWidth
      disableEscapeKeyDown={false}
      onClick={handleDialogClick} // Stop propagation for all clicks within dialog
    >
      <DialogTitle>Enter Customer MDOC</DialogTitle>
      <DialogContent>
        <TextField
          autoFocus
          margin="dense"
          id="mdoc"
          label="Customer MDOC"
          fullWidth
          variant="outlined"
          value={mdocInput}
          onChange={(e) => setMdocInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              handleSubmit();
            }
          }}
        />
        <Typography
          variant="caption"
          color="text.secondary"
          sx={{ mt: 1, display: "block" }}
        >
          Type the customer&apos;s MDOC (barcode ID) manually.
        </Typography>
      </DialogContent>
      <DialogActions>
        <Button onClick={handleClose}>Cancel</Button>
        <Button
          onClick={handleSubmit}
          variant="contained"
          disabled={!mdocInput || Number.isNaN(Number.parseInt(mdocInput, 10))}
        >
          Submit
        </Button>
      </DialogActions>
    </Dialog>
  );
}
