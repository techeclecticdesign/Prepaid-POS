import { invoke } from "@tauri-apps/api/core";
import type { OperatorDto } from "../../../hooks/useOperators";

export function useOperatorActions() {
  const create = async (dto: OperatorDto) => invoke("create_operator", { dto });
  const update = async (dto: OperatorDto) => invoke("update_operator", { dto });
  return { create, update };
}
