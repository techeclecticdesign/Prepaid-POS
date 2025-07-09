import React, { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type Operator from "./models/Operator";

interface AuthContextValue {
  loggedIn: boolean;
  login: (password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  timedOut: boolean;
  clearTimeoutFlag: () => void;
  activeOperator: Operator | null;
  setActiveOperator: (op: Operator | null) => void;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [loggedIn, setLoggedIn] = useState(false);
  const [timedOut, setTimedOut] = useState(false);
  const [activeOperator, setActiveOperator] = useState<Operator | null>(null);

  // Listen for any auth-status event from heartbeat hook
  useEffect(() => {
    const handler = (e: CustomEvent<boolean>) => {
      const ok = e.detail;
      if (!ok && loggedIn) {
        setTimedOut(true);
      }
      setLoggedIn(ok);
    };
    window.addEventListener("auth-status", handler as EventListener);
    return () => {
      window.removeEventListener("auth-status", handler as EventListener);
    };
  }, [loggedIn]);

  const login = async (password: string) => {
    try {
      await invoke("staff_login", { password });
      setLoggedIn(true);
      setTimedOut(false);
      return true;
    } catch {
      return false;
    }
  };

  const logout = async () => {
    try {
      await invoke("staff_logout");
    } finally {
      setLoggedIn(false);
      setTimedOut(false);
    }
  };

  const clearTimeoutFlag = () => {
    setTimedOut(false);
  };

  return (
    <AuthContext.Provider
      value={{
        loggedIn,
        login,
        logout,
        timedOut,
        clearTimeoutFlag,
        activeOperator,
        setActiveOperator,
      }}
    >
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
