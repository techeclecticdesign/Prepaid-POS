type OperatorThemePreferences = {
  [mdoc: number]: boolean;
};

type AdminThemePreference = boolean;

const STORAGE_KEY = "operatorThemePreferences";
const ADMIN_THEME_KEY = "adminThemePreference";

export function loadOperatorThemePreference(mdoc: number): boolean | null {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const preferences: OperatorThemePreferences = JSON.parse(stored);
      if (typeof preferences[mdoc] === "boolean") {
        return preferences[mdoc];
      }
    }
  } catch (error) {
    console.error("Failed to load theme preference from local storage:", error);
  }
  return null;
}

export function loadAdminThemePreference(): boolean | null {
  try {
    const stored = localStorage.getItem(ADMIN_THEME_KEY);
    if (stored) {
      const preference: AdminThemePreference = JSON.parse(stored);
      if (typeof preference === "boolean") {
        return preference;
      }
    }
  } catch (error) {
    console.error(
      "Failed to load admin theme preference from local storage:",
      error,
    );
  }
  return null;
}

export function saveOperatorThemePreference(
  mdoc: number,
  isDarkMode: boolean,
): void {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    const preferences: OperatorThemePreferences = stored
      ? JSON.parse(stored)
      : {};
    preferences[mdoc] = isDarkMode;
    localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences));
  } catch (error) {
    console.error("Failed to save theme preference to local storage:", error);
  }
}

export function saveAdminThemePreference(isDarkMode: boolean): void {
  localStorage.setItem(ADMIN_THEME_KEY, JSON.stringify(isDarkMode));
}
