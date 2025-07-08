import { useState, useEffect } from "react";
import type { Category } from "../../../models/Category";
import { useProductActions } from "./useProductActions";

export function useCategories() {
  const [categories, setCategories] = useState<Category[]>([]);
  const { listCategories } = useProductActions();
  useEffect(() => {
    listCategories().then(setCategories);
  }, []);
  return categories;
}
