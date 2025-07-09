import { useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Autocomplete from "@mui/material/Autocomplete";
import EditIcon from "@mui/icons-material/Edit";
import IconButton from "@mui/material/IconButton";
import DeleteIcon from "@mui/icons-material/Delete";
import ProductDialogLayout from "./ProductDialogLayout";
import EditPriceDialog from "./EditPriceDialog";
import useProductActions from "../hooks/useProductActions";
import type Product from "../../../models/Product";
import {
  editProductSchema,
  EditProductForm,
} from "../../../schema/productSchema";

interface Props {
  open: boolean;
  product: Product;
  categories: string[];
  onClose: () => void;
  onSave: (data: EditProductForm) => Promise<void>;
  onPriceAdjust: (
    oldPriceCents: number,
    newPriceCents: number,
  ) => Promise<void>;
  refetch: () => void;
}

export default function EditProductDialog({
  open,
  product,
  categories,
  onClose,
  onSave,
  onPriceAdjust,
  refetch,
}: Props) {
  const { removeProduct } = useProductActions();
  const { control, handleSubmit, reset, formState } = useForm<EditProductForm>({
    resolver: zodResolver(editProductSchema),
    defaultValues: {
      desc: product.desc,
      category: product.category,
    },
  });
  const [priceDialogOpen, setPriceDialogOpen] = useState(false);
  const [displayedPriceCents, setDisplayedPriceCents] = useState(product.price);

  // Reset form whenever product changes
  useEffect(() => {
    reset({ desc: product.desc, category: product.category });
    setDisplayedPriceCents(product.price);
  }, [product, reset]);

  const wrapped = handleSubmit(async (vals) => {
    await onSave(vals);
    onClose();
  });

  const handlePriceAdjustAndRefresh = async (
    oldPriceCents: number,
    newPriceCents: number,
  ) => {
    await onPriceAdjust(oldPriceCents, newPriceCents);
    setDisplayedPriceCents(newPriceCents);
  };

  const handleDelete = async () => {
    await removeProduct(product.upc);
    onClose();
    refetch(); // Update the list
  };

  return (
    <>
      <ProductDialogLayout
        open={open}
        title="Edit Product"
        onClose={onClose}
        onSubmit={wrapped}
        submitText="Save"
        leftActions={
          <Button onClick={handleDelete} color="error">
            <DeleteIcon /> Delete
          </Button>
        }
      >
        <div className="flex flex-col gap-4 pt-2">
          <TextField
            label="UPC"
            value={product.upc}
            slotProps={{ input: { readOnly: true } }}
            fullWidth
          />
          <div className="flex items-center space-x-2">
            <TextField
              label="Price"
              value={`$${(displayedPriceCents / 100).toFixed(2)}`}
              slotProps={{ input: { readOnly: true } }}
              fullWidth
            />
            <IconButton onClick={() => setPriceDialogOpen(true)}>
              <EditIcon />
            </IconButton>
          </div>
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
        </div>
      </ProductDialogLayout>
      <EditPriceDialog
        open={priceDialogOpen}
        initialPriceDollars={displayedPriceCents}
        onClose={() => setPriceDialogOpen(false)}
        onSubmit={handlePriceAdjustAndRefresh}
      />
    </>
  );
}
