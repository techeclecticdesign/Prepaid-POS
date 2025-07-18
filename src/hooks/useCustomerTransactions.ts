import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import type CustomerTransaction from "../models/CustomerTransaction";

export interface CustomerTransactionSearchRow {
  transaction: CustomerTransaction;
  operator_name: string;
  customer_name: string;
  spent: number;
}

export interface CustomerTransactionSearchResult {
  items: CustomerTransactionSearchRow[];
  total_count: number;
}

export default function useCustomerTransactions(
  mdoc: number,
  date: string | null,
  search: string,
  page: number,
) {
  const [transactions, setTransactions] = useState<
    CustomerTransactionSearchRow[]
  >([]);
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);

  const fetchTransactions = async () => {
    setLoading(true);
    try {
      const response = await invoke<CustomerTransactionSearchResult>(
        "search_customer_transactions",
        {
          mdoc,
          date: date || null,
          search: search || "",
          page,
        },
      );

      setTransactions(response.items);
      setTotalPages(Math.ceil(response.total_count / 10));
    } catch (error) {
      console.error("Failed to fetch customer transactions:", error);
      setTransactions([]);
      setTotalPages(1);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTransactions();
  }, [mdoc, date, search, page]);

  return {
    transactions,
    totalPages,
    loading,
    refetch: fetchTransactions,
  };
}
