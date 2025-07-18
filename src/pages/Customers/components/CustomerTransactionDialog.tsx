import { useState } from "react";
import { Pagination } from "@mui/material";
import Box from "@mui/material/Box";
import CircularProgress from "@mui/material/CircularProgress";
import Typography from "@mui/material/Typography";
import TableDialogLayout from "../../../components/TableDialogLayout";
import useCustomerTransactions from "../../../hooks/useCustomerTransactions";
import CustomerTransactionFilters from "./CustomerTransactionFilters";
import CustomerTransactionTable from "./CustomerTransactionTable";

interface Props {
  open: boolean;
  customerMdoc: number;
  customerName: string;
  onClose: () => void;
}

export default function CustomerTransactionDialog({
  open,
  customerMdoc,
  customerName,
  onClose,
}: Props) {
  const [date, setDate] = useState<Date | null>(null);
  const [page, setPage] = useState(1);

  const { transactions, totalPages, loading } = useCustomerTransactions(
    customerMdoc,
    date ? date.toISOString().split("T")[0] : null,
    "", // Search is not used here
    page,
  );

  const handleClose = () => {
    // Reset state when closing
    setDate(null);
    setPage(1);
    onClose();
  };

  return (
    <TableDialogLayout
      open={open}
      title={`Transactions for ${customerName}`}
      onClose={handleClose}
      onSubmit={handleClose}
      submitText="Close"
    >
      <Box sx={{ display: "flex", flexDirection: "column", height: "100%" }}>
        <CustomerTransactionFilters
          date={date}
          onDateChange={(newDate) => {
            setDate(newDate);
            setPage(1);
          }}
          isDialog={true}
        />

        {loading ? (
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
              flexGrow: 1,
            }}
          >
            <CircularProgress size={24} />
          </Box>
        ) : (
          <>
            <Box sx={{ flexGrow: 1, minHeight: 0 }}>
              <CustomerTransactionTable
                transactions={transactions}
                isDialog={true}
              />
            </Box>

            {totalPages > 1 && (
              <Box
                sx={{
                  display: "flex",
                  justifyContent: "center",
                  mt: 1.5,
                  flexShrink: 0,
                }}
              >
                <Pagination
                  count={totalPages}
                  page={page}
                  onChange={(_, v) => setPage(v)}
                  size="small"
                />
              </Box>
            )}
          </>
        )}

        {!loading && transactions.length === 0 && (
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
              flexGrow: 1,
            }}
          >
            <Typography variant="body2" color="text.secondary">
              No transactions found for this customer.
            </Typography>
          </Box>
        )}
      </Box>
    </TableDialogLayout>
  );
}
