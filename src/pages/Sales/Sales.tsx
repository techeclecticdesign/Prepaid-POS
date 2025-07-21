import { useRef, useEffect, useState, useCallback } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import { useTheme } from "@mui/material/styles";
import BarcodeScanner from "../../lib/barcode";
import { invoke } from "@tauri-apps/api/core";
import TransactionFooter from "./components/TransactionFooter";
import TransactionItems, {
  type TransactionItem,
} from "./components/TransactionItems";
import AppSnackbar from "../../components/AppSnackbar";
import CustomerMdocDialog from "./components/CustomerMdocDialog";
import UnknownUpcDialog from "./components/UnknownUpcDialog";
import { useAuth } from "../../AuthProvider";
import usePosInit, {
  type CustomerPosDto,
  type SaleDto,
  type SaleItemDto,
} from "./hooks/usePosInit";

type SnackbarSeverity = "error" | "warning" | "info" | "success";
type ScannerType = "Zebra" | "Generic";

export default function Sales() {
  const theme = useTheme();
  const { products, customers, loading, error, refetch } = usePosInit();
  const { activeOperator } = useAuth();

  // Transaction State
  const [selectedCustomer, setSelectedCustomer] =
    useState<CustomerPosDto | null>(null);
  const [transactionItems, setTransactionItems] = useState<TransactionItem[]>(
    [],
  );
  const [transactionTotal, setTransactionTotal] = useState(0);
  const [scannedUpc, setScannedUpc] = useState<string | null>(null);

  // Session State
  const [sessionSales, setSessionSales] = useState(0);
  const [sessionCustomers, setSessionCustomers] = useState(0);

  // UI State
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [snackbarSeverity, setSnackbarSeverity] =
    useState<SnackbarSeverity>("error");
  const [isMdocDialogOpen, setIsMdocDialogOpen] = useState(false);
  const [isUnknownUpcDialogOpen, setIsUnknownUpcDialogOpen] = useState(false);

  // Refs
  const errorTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const footerRef = useRef<HTMLDivElement>(null);

  // Utilities
  const getScannerType = (): ScannerType => {
    const saved = localStorage.getItem("barcode");
    const isValidType = (val: string | null): val is ScannerType =>
      val === "Zebra" || val === "Generic";

    if (isValidType(saved)) {
      return saved;
    }

    localStorage.setItem("barcode", "Zebra");
    return "Zebra";
  };

  const showSnackbar = (
    message: string,
    severity: SnackbarSeverity = "error",
  ) => {
    setSnackbarMessage(message);
    setSnackbarSeverity(severity);
    setSnackbarOpen(true);
  };

  const clearErrorTimer = () => {
    if (errorTimerRef.current) {
      clearTimeout(errorTimerRef.current);
    }
  };

  const endSession = () => {
    setSelectedCustomer(null);
    setTransactionTotal(0);
    setScannedUpc(null);
    setTransactionItems([]);
  };

  // Scan Handler
  const handleScan = useCallback(
    (code: string) => {
      clearErrorTimer();

      if (!/^\d+$/.test(code)) {
        showSnackbar("Invalid scan: Not a numeric code.", "error");
        return;
      }

      const scannedId = Number.parseInt(code, 10);

      if (!selectedCustomer) {
        // Customer scan mode
        const foundCustomer = customers.find(
          (cust) => cust.customer.mdoc === scannedId,
        );
        if (foundCustomer) {
          setSelectedCustomer(foundCustomer);
          setIsMdocDialogOpen(false);
        } else {
          showSnackbar(`Customer with MDOC #${scannedId} not found.`, "error");
        }
      } else {
        // Product scan mode
        const foundProduct = products.find((product) => product.upc === code);
        if (foundProduct) {
          setScannedUpc(code);
          setTimeout(() => setScannedUpc(null), 100);
        } else {
          setIsUnknownUpcDialogOpen(true);
        }
      }
    },
    [customers, selectedCustomer, products],
  );

  // Footer Button Handlers
  const handleAcceptReturn = () => {
    if (!selectedCustomer) {
      showSnackbar("Customer needs to scan in first.", "warning");
      return;
    }
    if (transactionItems.length !== 0) {
      showSnackbar("Cannot accept return when a sale is pending.", "warning");
      return;
    }
    console.log("Accept Return clicked");
  };

  const handleCancelTransaction = () => {
    if (!selectedCustomer) {
      showSnackbar("No active transaction to cancel.", "warning");
      return;
    }
    showSnackbar("Transaction cancelled.", "info");
    endSession();
  };

  const handleSubmitTransaction = async () => {
    if (!selectedCustomer || transactionItems.length === 0) {
      showSnackbar(
        "Cannot submit when missing purchases or customer.",
        "warning",
      );
      return;
    }

    const saleItems: SaleItemDto[] = transactionItems.map((item) => ({
      upc: item.upc,
      desc: item.name,
      quantity: item.quantity,
      price: item.price,
    }));

    const saleDto: SaleDto = {
      customer_mdoc: selectedCustomer.customer.mdoc,
      operator_mdoc: activeOperator?.mdoc ?? 0,
      operator_name: activeOperator?.name ?? "Unknown",
      customer_name: selectedCustomer.customer.name,
      items: saleItems,
    };

    try {
      const receiptPrinter = localStorage.getItem("receipt_printer") ?? "";
      const orderId = await invoke<number>("sale_transaction", {
        dto: saleDto,
        receiptPrinter: receiptPrinter,
      });

      // Update session stats
      setSessionSales((prev) => prev + transactionTotal);
      setSessionCustomers((prev) => prev + 1);

      endSession();
      refetch();

      showSnackbar(`Order #${orderId} submitted successfully!`, "success");
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      showSnackbar(`Failed to submit transaction: ${errorMessage}`);
      console.error("Failed to submit sale transaction:", err);
    }
  };

  // MDOC Dialog Handlers
  const handlePageClick = (event: React.MouseEvent) => {
    // Prevent opening dialog if a customer is already selected
    if (selectedCustomer) {
      return;
    }

    // Check if click is on footer
    if (footerRef.current && footerRef.current.contains(event.target as Node)) {
      return;
    }

    const target = event.target as HTMLElement;

    // Check for snackbar/toast elements
    if (
      target.closest(".MuiSnackbar-root") ||
      target.closest("[role='alert']") ||
      target.closest(".SnackbarContent-root")
    ) {
      return;
    }

    // Check for dialog elements
    if (
      target.closest(".MuiDialog-root") ||
      target.closest("[role='dialog']")
    ) {
      return;
    }

    // Check if the dialog is already open to prevent reopening
    if (isMdocDialogOpen) {
      return;
    }

    setIsMdocDialogOpen(true);
  };

  const handleCloseMdocDialog = () => {
    setIsMdocDialogOpen(false);
  };

  const handleSubmitMdoc = (mdoc: number) => {
    const foundCustomer = customers.find((cust) => cust.customer.mdoc === mdoc);
    if (foundCustomer) {
      setSelectedCustomer(foundCustomer);
      showSnackbar(`Customer #${mdoc} selected.`, "success");
    } else {
      showSnackbar(`Customer with MDOC #${mdoc} not found.`, "error");
    }
  };

  // Scanner Setup Effect
  useEffect(() => {
    const scannerType = getScannerType();
    const prefix = scannerType === "Zebra" ? "~" : "";

    const scanner = new BarcodeScanner({
      prefix,
      suffix: "",
      timeout: 50,
      shouldCapture: () => true,
      barcodeCallback: handleScan,
    });

    return () => {
      scanner.destroy();
      clearErrorTimer();
    };
  }, [handleScan]);

  // Loading State
  if (loading) {
    return (
      <Box className="min-h-screen flex items-center justify-center">
        <Typography variant="h5">Loading POS data...</Typography>
      </Box>
    );
  }

  // Error State
  if (error) {
    return (
      <Box className="min-h-screen flex flex-col relative">
        <Typography variant="h5" color="error">
          Error loading POS data: {error.message}
        </Typography>
      </Box>
    );
  }

  // Styles
  const customerInfoSx = {
    position: "absolute",
    top: theme.spacing(-1),
    left: theme.spacing(2),
    p: 1,
    borderRadius: theme.shape.borderRadius,
    backgroundColor: "transparent",
    color: "text.primary",
    transition: "background-color 0.3s ease-in-out",
    zIndex: 1,
  };

  const customerNameSx = {
    fontSize: {
      xs: theme.typography.pxToRem(20),
      md: theme.typography.pxToRem(24),
      lg: theme.typography.pxToRem(28),
    },
    fontWeight: theme.typography.fontWeightBold as number,
  };

  const customerMdocSx = {
    fontSize: {
      xs: theme.typography.pxToRem(12),
      md: theme.typography.pxToRem(14),
    },
    color: "text.secondary",
  };

  const waitingTextSx = {
    fontSize: {
      xs: theme.typography.pxToRem(18),
      md: theme.typography.pxToRem(22),
    },
    mt: "20%",
    color: "text.secondary",
  };

  // Main Render
  return (
    <Box
      onClick={handlePageClick}
      sx={{
        minHeight: "100vh",
        width: "100%",
        position: "relative",
      }}
    >
      <Box
        className="max-h-screen flex flex-col items-center justify-center relative"
        sx={{ pb: 12, flexGrow: 1 }}
      >
        {/* Customer Info Display */}
        <Box sx={customerInfoSx}>
          {selectedCustomer && (
            <>
              <Typography variant="h5" sx={customerNameSx}>
                {selectedCustomer.customer.name}
              </Typography>
              <Typography variant="body2" sx={customerMdocSx}>
                #{selectedCustomer.customer.mdoc}
              </Typography>
            </>
          )}
        </Box>

        {/* Main Content Area */}
        <Box
          className="flex-1 flex flex-col items-center justify-start pt-3"
          sx={{ width: "100%" }}
        >
          {selectedCustomer ? (
            <Box onClick={(e) => e.stopPropagation()}>
              <TransactionItems
                scannedUpc={scannedUpc}
                products={products}
                transactionItems={transactionItems}
                setTransactionItems={setTransactionItems}
                onTotalChange={setTransactionTotal}
              />
            </Box>
          ) : (
            <Typography variant="h6" sx={waitingTextSx}>
              Waiting for customer...
            </Typography>
          )}
        </Box>

        {/* Footer */}
        <Box
          ref={footerRef}
          onClick={(e) => e.stopPropagation()} // Prevent dialog from opening when clicking footer
        >
          <TransactionFooter
            selectedCustomer={selectedCustomer}
            transactionTotal={transactionTotal}
            sessionSales={sessionSales}
            sessionCustomers={sessionCustomers}
            onAcceptReturn={handleAcceptReturn}
            onCancelTransaction={handleCancelTransaction}
            onSubmitTransaction={handleSubmitTransaction}
          />
        </Box>
      </Box>

      {/* Snackbar Notifications */}
      <Box onClick={(e) => e.stopPropagation()}>
        <AppSnackbar
          open={snackbarOpen}
          message={snackbarMessage}
          severity={snackbarSeverity}
          onClose={() => setSnackbarOpen(false)}
        />
      </Box>

      {/* Unknown UPC Dialog */}
      <UnknownUpcDialog
        open={isUnknownUpcDialogOpen}
        onClose={() => setIsUnknownUpcDialogOpen(false)}
      />

      {/* Customer MDOC Dialog */}
      {isMdocDialogOpen && (
        <CustomerMdocDialog
          open={isMdocDialogOpen}
          onClose={handleCloseMdocDialog}
          onSubmitMdoc={handleSubmitMdoc}
        />
      )}
    </Box>
  );
}
