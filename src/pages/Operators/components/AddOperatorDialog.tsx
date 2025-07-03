import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from "@mui/material";
import { useForm, Controller } from "react-hook-form";
import {
  operatorSchema,
  OperatorFormValues,
} from "../../../schema/operatorSchema";
import { zodResolver } from "@hookform/resolvers/zod";
import TextField from "@mui/material/TextField";
import AppButton from "../../../components/AppButton";

interface Props {
  open: boolean;
  onClose: () => void;
  onSubmit: (values: OperatorFormValues) => void;
}

export default function AddOperatorDialog({ open, onClose, onSubmit }: Props) {
  const { control, handleSubmit, formState, reset } =
    useForm<OperatorFormValues>({
      resolver: zodResolver(operatorSchema),
    });

  return (
    <Dialog open={open} onClose={onClose}>
      <form
        onSubmit={handleSubmit((v) => {
          onSubmit(v);
          reset();
        })}
      >
        <DialogTitle>Add Operator</DialogTitle>
        <DialogContent className="space-y-4">
          <div className="flex flex-col gap-3">
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
          <AppButton text="Cancel" onClick={onClose} />
          <AppButton type="submit" variant="contained" text="Submit" />
        </DialogActions>
      </form>
    </Dialog>
  );
}
