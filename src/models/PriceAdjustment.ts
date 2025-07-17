export default interface PriceAdjustmentDto {
  id: number;
  operator_mdoc: number;
  upc: string;
  old: number;
  new: number;
  created_at: string; // RFC3339 string
}
