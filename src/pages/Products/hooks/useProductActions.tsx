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
  const searchProducts = async (
    search: string | null,
    category: string | null,
    page: number,
  ): Promise<SearchProductsResponse> => {
    return await invoke<SearchProductsResponse>("search_products", {
      search,
      category,
      page,
    });
  };

  const listCategories = async (): Promise<Category[]> => {
    return await invoke<Category[]>("list_categories");
  };

  const updateItem = async (
    upc: number,
    desc: string,
    category: string,
  ): Promise<void> => {
    await invoke("update_product", { upc, desc, category });
  };

  const priceAdjustment = async (dto: PriceAdjustmentDto): Promise<void> => {
    await invoke("price_adjustment", { dto });
  };

  const createProduct = async (dto: CreateProductDto): Promise<void> => {
    await invoke("create_product", { dto });
  };

  const removeProduct = async (upc: number): Promise<void> => {
    await invoke("delete_product", { upc });
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
