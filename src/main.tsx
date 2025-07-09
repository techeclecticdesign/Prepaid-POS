import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { AuthProvider, useAuth } from "./AuthProvider";
import { ThemeProvider } from "./theme/ThemeContext";
import "./tailwind.css";

// wrapper component to provide activeOperatorMdoc to ThemeProvider
function RootProviders() {
  const { activeOperator, loggedIn } = useAuth();

  return (
    <ThemeProvider
      activeOperatorMdoc={activeOperator ? activeOperator.id : null}
      isAdminLoggedIn={loggedIn}
    >
      <App />
    </ThemeProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AuthProvider>
      <RootProviders />
    </AuthProvider>
  </React.StrictMode>,
);
