import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import type ProductDto from "../../../models/Product";
import type CustomerDto from "../../../models/Customer";

export interface CustomerPosDto {
  customer: CustomerDto;
  balance: number;
}

export interface PosDto {
  products: ProductDto[];
  customers: CustomerPosDto[];
}

export interface SaleItemDto {
  upc: string;
  desc: string;
  quantity: number;
  price: number;
}

export interface SaleDto {
  customer_mdoc: number;
  operator_mdoc: number;
  operator_name: string;
  customer_name: string;
  items: SaleItemDto[];
}

export default function usePosInit() {
  const [products, setProducts] = useState<ProductDto[]>([]);
  const [customers, setCustomers] = useState<CustomerPosDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchPosData = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<PosDto>("pos_init");
      setProducts(result.products);
      setCustomers(result.customers);
    } catch (err) {
      console.error("Failed to load POS data:", err);
      setError(err as Error);
      setProducts([]);
      setCustomers([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosData();
  }, []);

  return {
    products,
    customers,
    loading,
    error,
    refetch: fetchPosData,
  };
}
