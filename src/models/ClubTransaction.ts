export default interface ClubTransaction {
  id: number;
  import_id: number | null;
  entity_name: string;
  mdoc: number | null;
  tx_type: string;
  amount: number;
  date: string; // RFC 3339
}
