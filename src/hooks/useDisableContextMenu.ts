// src/hooks/useDisableContextMenu.ts
import { useEffect } from "react";

export function useDisableContextMenu() {
  useEffect(() => {
    if (import.meta.env.DEV) return; // do nothing in dev

    const handler = (e: MouseEvent) => e.preventDefault();
    window.addEventListener("contextmenu", handler);

    return () => {
      window.removeEventListener("contextmenu", handler);
    };
  }, []);
}
