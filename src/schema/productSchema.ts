import { z } from "zod";

export const createProductSchema = z.object({
  upc: z.preprocess(
    (v) => {
      if (typeof v === "number") return v.toString();
      if (typeof v === "string") return v.trim();
      return v;
    },
    z
      .string()
      .nonempty("UPC is required")
      .regex(
        /^\d{8}$|^\d{12}$/,
        "UPC must be numerical and either 8 or 12 digits",
      ),
  ),
  price: z.preprocess(
    (v) => {
      if (typeof v === "string") {
        if (v.trim() === "") {
          return v;
        }
        const n = Number(v);
        return isNaN(n) ? v : Math.round(n * 100);
      }
      if (typeof v === "number") {
        return Math.round(v * 100);
      }
      return v;
    },
    z
      .number({
        required_error: "Price is required",
        invalid_type_error: "Price must be a number",
      })
      .int()
      .min(1, "Price must be greater than $0"),
  ),
  desc: z
    .string()
    .nonempty("Description is required")
    .transform((s) => s.trim()),
  category: z
    .string()
    .nonempty("Category is required")
    .transform((s) => s.trim()),
});

export const editProductSchema = z.object({
  desc: z
    .string()
    .nonempty("Description is required")
    .transform((s) => s.trim()),
  category: z
    .string()
    .nonempty("Category is required")
    .transform((s) => s.trim()),
});

export type EditProductForm = z.input<typeof editProductSchema>;
export type CreateProductForm = z.input<typeof createProductSchema>;
export type UpdateProduct = z.infer<typeof editProductSchema>;
export type CreateProduct = z.infer<typeof createProductSchema>;
