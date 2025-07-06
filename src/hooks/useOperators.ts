import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Operator } from "../models/Operator";

export function useOperators() {
  const [operators, setOperators] = useState<Operator[]>([]);

  const refresh = () => {
    invoke<Operator[]>("list_operators")
      .then(setOperators)
      .catch(console.error);
  };

  useEffect(refresh, []);

  return {
    operators,
    refresh,
  };
}
