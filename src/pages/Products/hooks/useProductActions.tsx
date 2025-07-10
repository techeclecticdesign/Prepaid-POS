import { invoke } from "@tauri-apps/api/core";
import type Product from "../../../models/Product";
import type Category from "../../../models/Category";

export interface SearchProductsResponse {
  products: Product[];
  total_count: number;
}

export interface PriceAdjustmentDto {
  operator_mdoc: number | undefined;
  upc: number;
  new: number;
  old: number;
  created_at: null;
}

export interface CreateProductDto {
  upc: number;
  desc: string;
  category: string;
  price: number;
}

export default function useProductActions() {
  const searchProducts = (
    search: string | null,
    category: string | null,
    page: number,
  ) => {
    return invoke<SearchProductsResponse>("search_products", {
      search,
      category,
      page,
    });
  };

  const listCategories = () => {
    return invoke<Category[]>("list_categories");
  };

  const updateItem = (upc: number, desc: string, category: string) => {
    return invoke("update_product", { upc, desc, category });
  };

  const priceAdjustment = (dto: PriceAdjustmentDto) => {
    return invoke("price_adjustment", { dto });
  };

  const createProduct = (dto: CreateProductDto) => {
    return invoke("create_product", { dto });
  };

  const removeProduct = (upc: number) => {
    return invoke("delete_product", { upc });
  };

  return {
    searchProducts,
    listCategories,
    updateItem,
    priceAdjustment,
    createProduct,
    removeProduct,
  };
}
