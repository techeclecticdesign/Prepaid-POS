import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface ClubTransaction {
  id: number;
  mdoc: number;
  amount: number;
  date: string; // RFC3339
}

export interface ClubTransactionSearchRow {
  transaction: ClubTransaction;
  customer_name: string;
}

export interface ClubTransactionSearchResult {
  items: ClubTransactionSearchRow[];
  total_count: number;
}

export default function useClubTransactions(
  search: string,
  date: string | null,
  page: number,
) {
  const [transactions, setTransactions] = useState<ClubTransactionSearchRow[]>(
    [],
  );
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);

  const fetchTransactions = async () => {
    setLoading(true);
    try {
      const response = await invoke<ClubTransactionSearchResult>(
        "search_club_transactions",
        {
          search: search || null,
          date: date || null,
          page,
        },
      );
      console.log(response);
      setTransactions(response.items);
      setTotalPages(Math.ceil(response.total_count / 10));
    } catch (error) {
      console.error("Failed to fetch club transactions:", error);
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
