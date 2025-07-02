import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useSessionPolling() {
  useEffect(() => {
    const checkStatus = async () => {
      const ok = await invoke<boolean>("check_login_status").catch(() => false);
      window.dispatchEvent(new CustomEvent("auth-status", { detail: ok }));
    };

    checkStatus();
    const interval = setInterval(checkStatus, 30_000);

    return () => clearInterval(interval);
  }, []);
}
