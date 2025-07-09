import React from "react";
import {
  createContext,
  useContext,
  useState,
  useEffect,
  type ReactNode,
} from "react";
import {
  loadOperatorThemePreference,
  saveOperatorThemePreference,
  loadAdminThemePreference,
  saveAdminThemePreference,
} from "./localStorageTheme";

type ThemeMode = "light" | "dark";

interface ThemeContextValue {
  mode: ThemeMode;
  toggleMode: () => void;
  setMode: (newMode: ThemeMode) => void;
}

const ThemeContext = createContext<ThemeContextValue>({
  mode: "light",
  toggleMode: () => {},
  setMode: () => {},
});

interface ThemeProviderProps {
  children: ReactNode;
  activeOperatorMdoc: number | null;
  isAdminLoggedIn: boolean;
}

export const useTheme = (): ThemeContextValue => useContext(ThemeContext);

export const ThemeProvider: React.FC<ThemeProviderProps> = ({
  children,
  activeOperatorMdoc,
  isAdminLoggedIn,
}) => {
  const [modeState, setModeState] = useState<ThemeMode>("light");

  const setMode = (newMode: ThemeMode) => {
    setModeState(newMode);
    if (activeOperatorMdoc !== null) {
      saveOperatorThemePreference(activeOperatorMdoc!, newMode === "dark");
    } else if (isAdminLoggedIn) {
      // Only save admin preference if no operator is active and admin is logged in
      saveAdminThemePreference(newMode === "dark");
    }
  };

  const toggleMode = () => {
    setMode(modeState === "light" ? "dark" : "light");
  };

  // Effect to load theme preference when activeOperatorMdoc or isAdminLoggedIn changes
  useEffect(() => {
    let preferredMode: boolean | null = null;
    if (activeOperatorMdoc !== null) {
      preferredMode = loadOperatorThemePreference(activeOperatorMdoc);
    }

    if (activeOperatorMdoc !== null) {
      const preferredMode = loadOperatorThemePreference(activeOperatorMdoc);
      if (preferredMode !== null) {
        setModeState(preferredMode ? "dark" : "light");
      } else {
        // If no operator preference, but an operator is active, default to light
        setModeState("light");
      }
    } else if (isAdminLoggedIn) {
      preferredMode = loadAdminThemePreference();
      if (preferredMode !== null) {
        setModeState(preferredMode ? "dark" : "light");
      } else {
        // If admin is logged in but no admin preference, default to light
        setModeState("light");
      }
    } else {
      setModeState("light");
    }
  }, [activeOperatorMdoc, isAdminLoggedIn]);

  // Add/remove the dark class on <html> when mode changes
  useEffect(() => {
    const htmlElement = document.documentElement;
    if (modeState === "dark") {
      htmlElement.classList.add("dark");
    } else {
      htmlElement.classList.remove("dark");
    }
  }, [modeState]);

  return (
    <ThemeContext.Provider value={{ mode: modeState, toggleMode, setMode }}>
      {children}
    </ThemeContext.Provider>
  );
};
