import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useCallback,
} from "react";
import { invoke } from "@tauri-apps/api/core";

interface AuthContextValue {
  loggedIn: boolean;
  login: (password: string) => Promise<boolean>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [loggedIn, setLoggedIn] = useState(false);

  // check login status
  const checkStatus = useCallback(async () => {
    try {
      const ok = await invoke<boolean>("check_login_status");
      setLoggedIn(ok);
    } catch {
      console.error("Auth check failed");
    }
  }, []);

  // polling
  useEffect(() => {
    checkStatus();
    const id = setInterval(checkStatus, 30_000);
    return () => clearInterval(id);
  }, [checkStatus]);

  const login = useCallback(
    async (password: string) => {
      try {
        await invoke("staff_login", { password });
        await checkStatus();
        return true;
      } catch {
        return false;
      }
    },
    [checkStatus],
  );

  const logout = useCallback(async () => {
    try {
      await invoke("staff_logout");
    } finally {
      setLoggedIn(false);
    }
  }, []);

  return (
    <AuthContext.Provider value={{ loggedIn, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextValue => {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error("useAuth must be used within AuthProvider");
  }
  return ctx;
};
