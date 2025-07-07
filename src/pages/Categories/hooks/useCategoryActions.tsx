import { invoke } from "@tauri-apps/api/core";

export function useCategoryActions() {
  const create = async (name: string): Promise<void> => {
    console.log("Create");
    await invoke("create_category", { name });
  };

  const remove = async (id: number): Promise<void> => {
    console.log("Delete");
    await invoke("delete_category", { id });
  };

  return { create, remove };
}
