import { invoke } from "@tauri-apps/api/core";
import type Operator from "../../../models/Operator";

export default function useOperatorActions() {
  const create = async (dto: Operator): Promise<void> => {
    await invoke("create_operator", { dto });
  };

  const update = async (dto: Operator): Promise<void> => {
    await invoke("update_operator", { dto });
  };
  return { create, update };
}
