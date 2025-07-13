import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import type Category from "../../../models/Category";

export default function useCategories() {
  const [categories, setCategories] = useState<Category[]>([]);

  const refresh = async () => {
    const list = await invoke<Category[]>("list_categories");
    setCategories(list);
  };

  useEffect(() => {
    refresh();
  }, []);

  return { categories, refresh };
}
