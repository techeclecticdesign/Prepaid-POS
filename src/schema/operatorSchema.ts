import { z } from "zod";

export const operatorSchema = z.object({
  mdoc: z.preprocess(
    (v) => (v === "" ? undefined : Number(v)),
    z.number().int().positive(),
  ),
  name: z
    .string()
    .nonempty()
    .refine((s) => s.trim().split(/\s+/).length >= 2),
});

export type OperatorFormValues = z.input<typeof operatorSchema>;
