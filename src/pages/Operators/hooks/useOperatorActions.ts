import { invoke } from "@tauri-apps/api/core";
import type Operator from "../../../models/Operator";

export default function useOperatorActions() {
  const create = async (dto: Operator) => invoke("create_operator", { dto });
  const update = async (dto: Operator) => invoke("update_operator", { dto });
  return { create, update };
}
