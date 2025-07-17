// kept name from backend model, although probably only to be used for LostInventory

export default interface InventoryTransactionDto {
  id: number | null;
  upc: string;
  quantity_change: number;
  reference: string | null;
  operator_mdoc: number;
  customer_mdoc: number | null;
  ref_order_id: number | null;
  created_at: string | null; // ISO/RFC3339 timestamp
}
