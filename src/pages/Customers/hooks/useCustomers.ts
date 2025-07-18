import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import Customer from "../../../models/Customer";

// Customers
export interface CustomerSearchRow {
  customer: Customer;
  balance: number;
}

export interface CustomerSearchResponse {
  customers: CustomerSearchRow[];
  total_count: number;
}

export default function useCustomers(search: string, page: number) {
  const [customers, setCustomers] = useState<CustomerSearchRow[]>([]);
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);

  const fetchCustomers = async () => {
    setLoading(true);
    try {
      const response = await invoke<CustomerSearchResponse>(
        "search_customers",
        {
          search: search || null,
          page,
        },
      );

      setCustomers(response.customers);
      setTotalPages(Math.ceil(response.total_count / 10));
    } catch (error) {
      console.error("Failed to fetch customers:", error);
      setCustomers([]);
      setTotalPages(1);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchCustomers();
  }, [search, page]);

  return {
    customers,
    totalPages,
    loading,
    refetch: fetchCustomers,
  };
}
