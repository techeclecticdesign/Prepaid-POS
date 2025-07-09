import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export default function useHandleActivity() {
  useEffect(() => {
    const handleActivity = async () => {
      await invoke("update_activity").catch(console.error);
      const ok = await invoke<boolean>("check_login_status").catch(() => false);
      window.dispatchEvent(new CustomEvent("auth-status", { detail: ok }));
    };

    window.addEventListener("click", handleActivity);
    window.addEventListener("keydown", handleActivity);

    return () => {
      window.removeEventListener("click", handleActivity);
      window.removeEventListener("keydown", handleActivity);
    };
  }, []);
}
