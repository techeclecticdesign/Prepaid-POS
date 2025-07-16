import { useState, useEffect } from "react";
import type Product from "../../../models/Product";
import useProductActions from "./useProductActions";

export default function useProducts(
  search: string,
  category: string,
  page: number,
) {
  const [products, setProducts] = useState<
    Array<Product & { available: number }>
  >([]);
  const [totalPages, setTotalPages] = useState(1);
  const { searchProducts } = useProductActions();

  // wrap our fetch logic so we can re-use it
  const refetch = () => {
    searchProducts(search || null, category || null, page).then((res) => {
      setProducts(res.products);
      setTotalPages(Math.ceil(res.total_count / 10));
    });
  };

  // initial + deps-driven fetch
  useEffect(() => {
    refetch();
  }, [search, category, page]);

  return { products, totalPages, refetch };
}
