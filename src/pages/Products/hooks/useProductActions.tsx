import { invoke } from "@tauri-apps/api/core";
import type Product from "../../../models/Product";
import type Category from "../../../models/Category";

export interface SearchProductsResponse {
  products: Product[];
  total_count: number;
}

export interface PriceAdjustmentDto {
  operator_mdoc: number | undefined;
  upc: string;
  new: number;
  old: number;
  created_at: null;
}

export interface CreateProductDto {
  upc: string;
  desc: string;
  category: string;
  price: number;
}

export interface UpdateProductDto {
  upc: string;
  desc: string;
  category: string;
}

export interface DeleteProductDto {
  upc: string;
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

  const updateItem = async (dto: UpdateProductDto): Promise<void> => {
    await invoke("update_product", {
      dto,
    });
  };

  const priceAdjustment = async (dto: PriceAdjustmentDto): Promise<void> => {
    await invoke("price_adjustment", { dto });
  };

  const createProduct = async (dto: CreateProductDto): Promise<void> => {
    await invoke("create_product", { dto });
  };

  const removeProduct = async (dto: DeleteProductDto): Promise<void> => {
    await invoke("delete_product", { dto });
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
