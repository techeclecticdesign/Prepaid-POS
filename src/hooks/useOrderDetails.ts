import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

export interface CustomerTxDetailDto {
  detail_id: number;
  order_id: number;
  upc: string;
  product_name: string;
  quantity: number;
  price: number;
}

export default function useOrderDetails(orderId: number) {
  const [details, setDetails] = useState<CustomerTxDetailDto[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchDetails = async () => {
    setLoading(true);
    try {
      const response = await invoke<CustomerTxDetailDto[]>(
        "list_order_details",
        {
          orderId,
        },
      );

      setDetails(response);
    } catch (error) {
      console.error("Failed to fetch order details:", error);
      setDetails([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (orderId) {
      fetchDetails();
    }
  }, [orderId]);

  return {
    details,
    loading,
    refetch: fetchDetails,
  };
}
