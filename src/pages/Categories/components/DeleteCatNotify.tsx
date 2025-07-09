import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Typography from "@mui/material/Typography";
import AppButton from "../../../components/AppButton";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function DeleteCatNotify({ open, onClose }: Props) {
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>Delete Category</DialogTitle>
      <DialogContent>
        <Typography variant="body2" sx={{ color: "text.secondary" }}>
          Deleting a category will not alter products already assigned to the
          deleted category. Product reassignment to different categories must be
          done manually.
        </Typography>
      </DialogContent>
      <DialogActions>
        <AppButton onClick={onClose} text="Ok" />
      </DialogActions>
    </Dialog>
  );
}
