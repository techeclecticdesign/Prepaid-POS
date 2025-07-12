import { BrowserRouter, Routes, Route } from "react-router-dom";
import { useAuth } from "./AuthProvider";
import SidebarLayout from "./layouts/SidebarLayout";
import AdminSidebar from "./components/AdminSidebar";
import OperatorSidebar from "./components/OperatorSidebar";
import AdminPage from "./pages/AdminPage/AdminPage";
import CategoriesPage from "./pages/Categories/Categories";
import OperatorsPage from "./pages/Operators/Operators";
import ProductsPage from "./pages/Products/Products";
import SalesPage from "./pages/Sales/Sales";
import Homepage from "./pages/Homepage/Homepage";
import ImportPage from "./pages/Import/Import";

export default function App() {
  const { loggedIn, activeOperator } = useAuth();

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Homepage />} />
        {loggedIn && (
          <Route
            element={
              <SidebarLayout
                Sidebar={AdminSidebar}
                activeOperatorMdoc={activeOperator ? activeOperator.id : null}
              />
            }
          >
            <Route path="/admin" element={<AdminPage />} />
            <Route path="/operators" element={<OperatorsPage />} />
            <Route path="/categories" element={<CategoriesPage />} />
            <Route path="/import" element={<ImportPage />} />
          </Route>
        )}
        {activeOperator && (
          <Route
            element={
              <SidebarLayout
                Sidebar={OperatorSidebar}
                activeOperatorMdoc={activeOperator.id}
              />
            }
          >
            <Route path="/products" element={<ProductsPage />} />
            <Route path="/sales" element={<SalesPage />} />
          </Route>
        )}
      </Routes>
    </BrowserRouter>
  );
}
