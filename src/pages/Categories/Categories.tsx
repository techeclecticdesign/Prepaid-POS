import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import useCategories from "./hooks/useCategories";
import useCategoryActions from "./hooks/useCategoryActions";
import CategoryList from "./components/CategoryList";
import AddCategoryDialog from "./components/AddCategoryDialog";
import DeleteCatNotify from "./components/DeleteCatNotify";
import AppButton from "../../components/AppButton";

export default function CategoriesPage() {
  const { categories, refresh } = useCategories();
  const { create, remove } = useCategoryActions();
  const [showAdd, setShowAdd] = useState<boolean>(false);
  const [notifyOpen, setNotifyOpen] = useState<boolean>(false);

  const handleAdd = async (name: string) => {
    await create(name);
    setShowAdd(false);
    refresh();
  };

  const handleDelete = async (id: number) => {
    setNotifyOpen(true);
    await remove(id);
    refresh();
  };

  return (
    <Box className="fixed top-0 w-[calc(100vw-21.5rem)] h-screen overflow-auto">
      <Box className="p-12 flex justify-center">
        <Box className="w-110">
          <Typography
            variant="h4"
            component="h1"
            className="font-bold text-center"
            sx={{ color: "text.primary" }}
          >
            Categories
          </Typography>
          <Box className="my-12 text-center">
            <AppButton onClick={() => setShowAdd(true)} text="Add Category" />
          </Box>
          <CategoryList categories={categories} onDelete={handleDelete} />
          <AddCategoryDialog
            open={showAdd}
            onClose={() => setShowAdd(false)}
            onSubmit={handleAdd}
            existingNames={categories.map((c) => c.name)}
          />
          <DeleteCatNotify
            open={notifyOpen}
            onClose={() => setNotifyOpen(false)}
          />
        </Box>
      </Box>
    </Box>
  );
}
