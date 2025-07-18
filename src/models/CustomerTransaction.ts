export default interface CustomerTransaction {
  order_id: number;
  customer_mdoc: number;
  operator_mdoc: number;
  date: string; // RFC3339
  note?: string;
}
