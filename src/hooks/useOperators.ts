import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface OperatorDto {
  id: number;
  name: string;
  start: string;
  stop: string | null;
}

export function useOperators() {
  const [operators, setOperators] = useState<OperatorDto[]>([]);

  const refresh = () => {
    invoke<OperatorDto[]>("list_operators")
      .then(setOperators)
      .catch(console.error);
  };

  useEffect(refresh, []);

  return {
    operators,
    refresh,
  };
}
