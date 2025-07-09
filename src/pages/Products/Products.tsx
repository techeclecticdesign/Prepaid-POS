import { useState } from "react";
import { Pagination, IconButton } from "@mui/material";
import AddIcon from "@mui/icons-material/Add";
import type Product from "../../models/Product";
import useProducts from "./hooks/useProducts";
import useCategories from "./hooks/useCategories";
import useProductActions from "./hooks/useProductActions";
import { useAuth } from "../../AuthProvider";
import ProductsTable from "./components/ProductsTable";
import ProductsFilters from "./components/ProductsFilters";
import EditProductDialog from "./components/EditProductDialog";
import CreateProductDialog from "./components/CreateProductDialog";

export default function ProductsPage() {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState("");
  const [editing, setEditing] = useState<Product | null>(null);
  const [creating, setCreating] = useState(false);
  const [page, setPage] = useState(1);
  const { products, totalPages, refetch } = useProducts(search, category, page);
  const categories = useCategories();
  const { activeOperator } = useAuth();
  const { updateItem, priceAdjustment, createProduct } = useProductActions();

  return (
    <div className="p-8 min-h-screen w-3/5">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">Products</h1>
        <IconButton onClick={() => setCreating(true)}>
          <AddIcon className="mt-1" />
        </IconButton>
      </div>
      <ProductsFilters
        search={search}
        category={category}
        categories={categories}
        onSearchChange={(newSearch) => {
          setSearch(newSearch);
          setPage(1);
        }}
        onCategoryChange={(newCategory) => {
          setCategory(newCategory);
          setPage(1);
        }}
      />
      <ProductsTable products={products} onProductClick={setEditing} />

      <div className="flex justify-center mt-4">
        <Pagination
          count={totalPages}
          page={page}
          onChange={(_, v) => setPage(v)}
        />
      </div>

      {editing && (
        <EditProductDialog
          open={true}
          product={editing}
          categories={categories.map((c) => c.name)}
          onClose={() => setEditing(null)}
          onSave={async (vals) => {
            await updateItem(editing.upc, vals.desc, vals.category);
            refetch();
          }}
          onPriceAdjust={async (oldCents, newCents) => {
            await priceAdjustment({
              operator_mdoc: activeOperator?.id,
              upc: editing.upc,
              new: newCents,
              old: oldCents,
              created_at: null,
            });
            refetch();
          }}
          refetch={refetch}
        />
      )}

      <CreateProductDialog
        open={creating}
        categories={categories.map((c) => c.name)}
        onClose={() => setCreating(false)}
        onCreate={async (vals) => {
          const upcNum = Number(vals.upc);
          const priceCents = Math.round(Number(vals.price));
          await createProduct({
            upc: upcNum,
            desc: vals.desc,
            category: vals.category,
            price: priceCents,
          });
          refetch();
          setCreating(false);
        }}
      />
    </div>
  );
}
