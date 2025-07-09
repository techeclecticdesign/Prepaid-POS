import { useState } from "react";
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
    <div className="p-12 flex justify-center w-full h-full">
      <div className="w-2/3">
        <h1 className="text-4xl font-bold mb-8 text-center">Categories</h1>

        <div className="mb-12 text-center">
          <AppButton onClick={() => setShowAdd(true)} text="Add Category" />
        </div>

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
      </div>
    </div>
  );
}
