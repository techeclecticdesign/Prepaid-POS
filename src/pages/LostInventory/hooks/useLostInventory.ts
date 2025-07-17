import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import InventoryTransactionDto from "../../../models/InventoryTransaction";

export interface LostInventorySearchRow {
  transaction: InventoryTransactionDto;
  product_name: string;
  operator_name: string;
}

export interface LostInventoryResponse {
  transactions: LostInventorySearchRow[];
  total_count: number;
}

export default function useLostInventory(
  search: string,
  date: string | null,
  page: number,
) {
  const [transactions, setTransactions] = useState<LostInventorySearchRow[]>(
    [],
  );
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);

  const fetchTransactions = async () => {
    setLoading(true);
    try {
      const response = await invoke<LostInventoryResponse>(
        "search_inventory_transactions",
        {
          search: search || null,
          date: date || null,
          page,
        },
      );

      setTransactions(response.transactions);
      setTotalPages(Math.ceil(response.total_count / 10));
    } catch (error) {
      console.error("Failed to fetch inventory transactions:", error);
      setTransactions([]);
      setTotalPages(1);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTransactions();
  }, [search, date, page]);

  return {
    transactions,
    totalPages,
    loading,
    refetch: fetchTransactions,
  };
}
