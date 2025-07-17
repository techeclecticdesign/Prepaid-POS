import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

import PriceAdjustmentDto from "../../../models/PriceAdjustment";

export interface PriceAdjustmentSearchRow {
  adjustment: PriceAdjustmentDto;
  operator_name: string;
  product_name: string;
}

export interface PriceAdjustmentSearchResult {
  adjustments: PriceAdjustmentSearchRow[];
  total_count: number;
}

export default function usePriceAdjustments(
  search: string,
  date: string | null,
  page: number,
) {
  const [adjustments, setAdjustments] = useState<PriceAdjustmentSearchRow[]>(
    [],
  );
  const [totalPages, setTotalPages] = useState(1);
  const [loading, setLoading] = useState(false);

  const fetchAdjustments = async () => {
    setLoading(true);
    try {
      const response = await invoke<PriceAdjustmentSearchResult>(
        "search_price_adjustments",
        {
          page,
          date: date || null,
          search: search || null,
        },
      );

      setAdjustments(response.adjustments);
      setTotalPages(Math.ceil(response.total_count / 10));
    } catch (error) {
      console.error("Failed to fetch price adjustments:", error);
      setAdjustments([]);
      setTotalPages(1);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchAdjustments();
  }, [search, date, page]);

  return {
    adjustments,
    totalPages,
    loading,
    refetch: fetchAdjustments,
  };
}
