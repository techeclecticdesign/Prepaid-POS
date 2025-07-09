import { useState } from "react";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import TextField from "@mui/material/TextField";
import { Controller, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import AppButton from "../../../components/AppButton";
import AppSnackbar from "../../../components/AppSnackbar";
import {
  operatorSchema,
  type OperatorFormValues,
} from "../../../schema/operatorSchema";

interface Props {
  open: boolean;
  onClose: () => void;
  onSubmit: (values: OperatorFormValues) => Promise<void>;
  existingMdocs: number[];
}

export default function AddOperatorDialog({
  open,
  onClose,
  onSubmit,
  existingMdocs,
}: Props) {
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMsg, setSnackbarMsg] = useState("");
  const { control, handleSubmit, formState, reset } =
    useForm<OperatorFormValues>({
      resolver: zodResolver(operatorSchema),
    });

  // wrap form submit to catch errors
  const wrappedSubmit = handleSubmit(async (vals) => {
    if (existingMdocs.some((m) => m === vals.mdoc)) {
      setSnackbarMsg("Operator with that MDOC already exists.");
      setSnackbarOpen(true);
      return;
    }
    try {
      await onSubmit(vals);
      onClose();
      reset();
    } catch (err) {
      setSnackbarMsg(err instanceof Error ? err.message : String(err));
      setSnackbarOpen(true);
    }
  });

  return (
    <>
      <Dialog open={open} onClose={onClose}>
        <form onSubmit={wrappedSubmit}>
          <DialogTitle>Add Operator</DialogTitle>
          <DialogContent className="space-y-4">
            <div className="flex flex-col gap-3 pt-1.5">
              <Controller
                name="mdoc"
                control={control}
                render={({ field }) => (
                  <TextField
                    {...field}
                    label="MDOC"
                    fullWidth
                    error={!!formState.errors.mdoc}
                    helperText={formState.errors.mdoc?.message}
                  />
                )}
              />
              <Controller
                name="name"
                control={control}
                defaultValue=""
                render={({ field }) => (
                  <TextField
                    {...field}
                    label="Full Name"
                    fullWidth
                    error={!!formState.errors.name}
                    helperText={formState.errors.name?.message}
                  />
                )}
              />
            </div>
          </DialogContent>
          <DialogActions>
            <AppButton text="Cancel" onClick={onClose} variant="outlined" />
            <AppButton type="submit" text="Submit" />
          </DialogActions>
        </form>
      </Dialog>
      <AppSnackbar
        open={snackbarOpen}
        message={snackbarMsg}
        onClose={() => setSnackbarOpen(false)}
      />
    </>
  );
}
