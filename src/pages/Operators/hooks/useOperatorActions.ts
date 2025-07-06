import { invoke } from "@tauri-apps/api/core";
import type { Operator } from "../../../models/Operator";

export function useOperatorActions() {
  const create = async (op: Operator) => invoke("create_operator", { op });
  const update = async (op: Operator) => invoke("update_operator", { op });
  return { create, update };
}
