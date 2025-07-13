import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type Operator from "../../../models/Operator";

interface UseLegacyDataCheckResult {
  shouldPromptForLegacyData: boolean;
  acknowledgePrompt: () => void;
}

export function useLegacyDataCheck(
  operators: Operator[],
  isLoadingOperators: boolean,
): UseLegacyDataCheckResult {
  const [shouldPrompt, setShouldPrompt] = useState(false);
  const [hasChecked, setHasChecked] = useState(false);

  useEffect(() => {
    // Only run if operators loaded, there are none, and we haven't checked yet
    if (!isLoadingOperators && operators.length === 0 && !hasChecked) {
      const checkLegacyData = async () => {
        try {
          const hasLegacy = await invoke<boolean>("has_legacy_data");
          if (hasLegacy) {
            setShouldPrompt(true);
          }
        } catch (err) {
          console.error("Error checking for legacy data:", err);
        } finally {
          setHasChecked(true);
        }
      };
      void checkLegacyData();
    }
  }, [isLoadingOperators, operators.length, hasChecked]);

  const acknowledgePrompt = () => {
    setShouldPrompt(false);
  };

  return {
    shouldPromptForLegacyData: shouldPrompt,
    acknowledgePrompt,
  };
}
