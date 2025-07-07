import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Category } from "../../../models/Category";

export function useCategories() {
  const [categories, setCategories] = useState<Category[]>([]);

  const refresh = async () => {
    const list = await invoke<Category[]>("list_categories");
    console.log(list);
    setCategories(list);
  };

  useEffect(() => {
    refresh();
  }, []);

  return { categories, refresh };
}
