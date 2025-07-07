import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Button from "@mui/material/Button";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function DeleteCatNotify({ open, onClose }: Props) {
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>Delete Category</DialogTitle>
      <DialogContent>
        <p className="text-sm text-gray-700">
          Deleting a category will not alter products already assigned to the
          deleted category. Product reassignment to different categories must be
          done manually.
        </p>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose} variant="contained">
          OK
        </Button>
      </DialogActions>
    </Dialog>
  );
}
