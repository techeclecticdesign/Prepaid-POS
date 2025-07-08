import { useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import TextField from "@mui/material/TextField";
import Autocomplete from "@mui/material/Autocomplete";
import ProductDialogLayout from "./ProductDialogLayout";
import AppSnackbar from "../../../components/AppSnackbar";
import {
  createProductSchema,
  CreateProductForm,
} from "../../../schema/productSchema";

interface Props {
  open: boolean;
  categories: string[];
  onClose: () => void;
  onCreate: (data: CreateProductForm) => Promise<void>;
}

export default function CreateProductDialog({
  open,
  categories,
  onClose,
  onCreate,
}: Props) {
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [snackbarSeverity, setSnackbarSeverity] = useState<
    "error" | "warning" | "info" | "success"
  >("error");
  const { control, handleSubmit, reset, formState } =
    useForm<CreateProductForm>({
      resolver: zodResolver(createProductSchema),
      defaultValues: { upc: "", price: "", desc: "", category: "" },
    });

  // Reset form whenever product changes
  useEffect(() => {
    reset({ upc: "", price: "", desc: "", category: "" });
    setSnackbarOpen(false);
  }, [open, reset]);

  const wrapped = handleSubmit(async (vals) => {
    try {
      await onCreate(vals);
      onClose();
    } catch (error) {
      console.error("Failed to create product:", error);
      setSnackbarMessage(
        `Failed to create product: ${error instanceof Error ? error.message : String(error)}`,
      );
      setSnackbarSeverity("error");
      setSnackbarOpen(true);
    }
  });

  const handleCloseSnackbar = (
    _event?: React.SyntheticEvent | Event,
    reason?: string,
  ) => {
    if (reason === "clickaway") {
      return;
    }
    setSnackbarOpen(false);
  };

  return (
    <>
      <ProductDialogLayout
        open={open}
        title="New Product"
        onClose={() => {
          reset();
          onClose();
        }}
        onSubmit={wrapped}
        submitText="Create"
      >
        <div className="flex flex-col gap-4 pt-2">
          <Controller
            name="desc"
            control={control}
            render={({ field }) => (
              <TextField
                {...field}
                label="Description"
                error={!!formState.errors.desc}
                helperText={formState.errors.desc?.message}
                fullWidth
              />
            )}
          />
          <Controller
            name="category"
            control={control}
            render={({ field }) => (
              <Autocomplete
                {...field}
                options={categories}
                value={field.value}
                onChange={(_, v) => field.onChange(v ?? "")}
                renderInput={(params) => (
                  <TextField
                    {...params}
                    label="Category"
                    error={!!formState.errors.category}
                    helperText={formState.errors.category?.message}
                    fullWidth
                  />
                )}
              />
            )}
          />
          <Controller
            name="upc"
            control={control}
            render={({ field }) => (
              <TextField
                {...field}
                label="Scan UPC"
                error={!!formState.errors.upc}
                helperText={formState.errors.upc?.message}
                fullWidth
              />
            )}
          />
          <Controller
            name="price"
            control={control}
            render={({ field }) => (
              <TextField
                {...field}
                label="Price ($)"
                type="number"
                error={!!formState.errors.price}
                helperText={formState.errors.price?.message}
                fullWidth
              />
            )}
          />
        </div>
      </ProductDialogLayout>
      <AppSnackbar
        open={snackbarOpen}
        message={snackbarMessage}
        severity={snackbarSeverity}
        onClose={handleCloseSnackbar}
      />
    </>
  );
}
