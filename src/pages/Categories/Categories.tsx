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
    <Box
      sx={{
        position: "fixed",
        top: 0,
        left: 0,
        right: "18rem",
        bottom: 0,
        overflowY: "auto",
        /* custom scrollbar */
        "&::-webkit-scrollbar": {
          width: "10px",
        },
        "&::-webkit-scrollbar-track": {
          background: "transparent",
        },
        "&::-webkit-scrollbar-thumb": (theme) => ({
          borderRadius: "8px",
          backgroundColor:
            theme.palette.mode === "dark"
              ? "rgba(255,255,255,0.3)"
              : "rgba(0,0,0,0.3)",
          border: "2px solid transparent",
          backgroundClip: "padding-box",
        }),
        scrollbarWidth: "thin",
        scrollbarColor: (theme) =>
          theme.palette.mode === "dark"
            ? "rgba(255,255,255,0.3) transparent"
            : "rgba(0,0,0,0.3) transparent",
      }}
    >
      <Box sx={{ p: 12, display: "flex", justifyContent: "center" }}>
        <Box sx={{ width: 440 }}>
          <Typography
            variant="h4"
            component="h1"
            sx={{
              color: "text.primary",
              fontWeight: "bold",
              textAlign: "center",
            }}
          >
            Categories
          </Typography>
          <Box sx={{ my: 12, textAlign: "center" }}>
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
