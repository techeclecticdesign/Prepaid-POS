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
import LostInventoryPage from "./pages/LostInventory/LostInventory";
import PriceAdjustmentsPage from "./pages/PriceAdjustments/PriceAdjustments";
import CustomersPage from "./pages/Customers/Customers";
import AccountsPage from "./pages/Accounts/Accounts";
import BarcodeConfig from "./pages/BarcodeConfig/BarcodeConfig";
import PrinterConfigPage from "./pages/PrinterConfig/PrinterConfig";
import ChangePasswordPage from "./pages/Password/Password";
import ReportsPage from "./pages/Reports/Reports";
import WeeklyLimitPage from "./pages/WeeklyLimit/WeeklyLimit";

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
                activeOperatorMdoc={activeOperator ? activeOperator.mdoc : null}
              />
            }
          >
            <Route path="/admin" element={<AdminPage />} />
            <Route path="/operators" element={<OperatorsPage />} />
            <Route path="/categories" element={<CategoriesPage />} />
            <Route path="/lost-inventory" element={<LostInventoryPage />} />
            <Route path="/barcode" element={<BarcodeConfig />} />
            <Route path="/printer" element={<PrinterConfigPage />} />
            <Route path="/change-password" element={<ChangePasswordPage />} />
            <Route path="/weekly-limit" element={<WeeklyLimitPage />} />
            <Route
              path="/price-adjustments"
              element={<PriceAdjustmentsPage />}
            />
            <Route path="/import" element={<ImportPage />} />
          </Route>
        )}
        {activeOperator && (
          <Route
            element={
              <SidebarLayout
                Sidebar={OperatorSidebar}
                activeOperatorMdoc={activeOperator.mdoc}
              />
            }
          >
            <Route path="/products" element={<ProductsPage />} />
            <Route path="/sales" element={<SalesPage />} />
            <Route path="/customers" element={<CustomersPage />} />
            <Route path="/accounts" element={<AccountsPage />} />
            <Route path="/reports" element={<ReportsPage />} />
          </Route>
        )}
      </Routes>
    </BrowserRouter>
  );
}
