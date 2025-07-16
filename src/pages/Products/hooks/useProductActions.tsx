import { invoke } from "@tauri-apps/api/core";
import type Product from "../../../models/Product";
import type Category from "../../../models/Category";

export interface ProductSearchRow {
  product: Product;
  available: number;
}

export interface SearchProductsResponse {
  products: ProductSearchRow[];
  total_count: number;
}

export interface PriceAdjustmentDto {
  operator_mdoc: number;
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

export interface InventoryAdjustmentDto {
  operator_mdoc: number;
  upc: string;
  quantity_change: number;
  reference?: string;
  customer_mdoc?: number;
  ref_order_id?: number;
  created_at?: string;
}

export default function useProductActions() {
  const searchProducts = async (
    search: string | null,
    category: string | null,
    page: number,
  ): Promise<{ products: Product[]; total_count: number }> => {
    const res = await invoke<SearchProductsResponse>("search_products", {
      search,
      category,
      page,
    });

    return {
      total_count: res.total_count,
      products: res.products.map(({ product, available }) => ({
        ...product,
        available,
      })),
    };
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

  const quantityAdjustment = async (
    dto: InventoryAdjustmentDto,
  ): Promise<void> => {
    await invoke("inventory_adjustment", { dto });
  };

  return {
    searchProducts,
    listCategories,
    updateItem,
    priceAdjustment,
    createProduct,
    removeProduct,
    quantityAdjustment,
  };
}
