import { useState } from "react";
import { Pagination, IconButton } from "@mui/material";
import AddIcon from "@mui/icons-material/Add";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
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
    <Box className="p-2 w-3/5 mx-auto 2xl:px-50">
      <Box className="flex justify-between items-center mb-6">
        <Typography
          variant="h4"
          component="h1"
          className="font-bold"
          sx={{ color: "text.primary" }}
        >
          Products
        </Typography>
        <IconButton onClick={() => setCreating(true)}>
          <AddIcon className="mt-1" />
        </IconButton>
      </Box>
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
      <Box className="flex justify-center mt-4 2xl:mt-12">
        <Pagination
          count={totalPages}
          page={page}
          onChange={(_, v) => setPage(v)}
        />
      </Box>
      {editing && (
        <EditProductDialog
          open={true}
          product={editing}
          categories={categories.map((c) => c.name)}
          onClose={() => setEditing(null)}
          onSave={async (vals) => {
            await updateItem({
              upc: editing.upc,
              desc: vals.desc,
              category: vals.category,
            });
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
          const priceCents = Math.round(Number(vals.price));
          await createProduct({
            upc: vals.upc as string,
            desc: vals.desc,
            category: vals.category,
            price: priceCents,
          });
          refetch();
          setCreating(false);
        }}
      />
    </Box>
  );
}
