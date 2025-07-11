import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type Operator from "../models/Operator";

export default function useOperators() {
  const [operators, setOperators] = useState<Operator[]>([]);

  const refresh = async (): Promise<void> => {
    try {
      const result = await invoke<Operator[]>("list_operators");
      setOperators(result);
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    void refresh();
  }, []);

  return {
    operators,
    refresh,
  };
}
