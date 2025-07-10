import { invoke } from "@tauri-apps/api/core";

export default function useCategoryActions() {
  const create = async (name: string): Promise<void> => {
    await invoke("create_category", { dto: { name } });
  };

  const remove = async (id: number): Promise<void> => {
    await invoke("delete_category", { dto: { id } });
  };

  return { create, remove };
}
