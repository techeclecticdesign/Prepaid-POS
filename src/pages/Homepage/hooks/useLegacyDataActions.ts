import { invoke } from "@tauri-apps/api/core";

export default function useLegacyDataActions() {
  const checkLegacyDataExists = async (): Promise<boolean> => {
    try {
      const hasLegacy = await invoke<boolean>("has_legacy_data");
      return hasLegacy;
    } catch (err) {
      console.error("Error invoking has_legacy_data:", err);
      return false;
    }
  };

  const importLegacyData = async (): Promise<void> => {
    try {
      await invoke("do_legacy_data_import");
      console.log("Legacy data import command invoked successfully.");
    } catch (err) {
      console.error("Error invoking do_legacy_data_import:", err);
    }
  };

  return {
    checkLegacyDataExists,
    importLegacyData,
  };
}
