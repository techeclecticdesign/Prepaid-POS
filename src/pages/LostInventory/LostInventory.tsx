import { useState } from "react";
import { Pagination } from "@mui/material";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import CircularProgress from "@mui/material/CircularProgress";
import useLostInventory from "./hooks/useLostInventory";
import LostInventoryFilters from "./components/LostInventoryFilters";
import LostInventoryTable from "./components/LostInventoryTable";

export default function LostInventory() {
  const [search, setSearch] = useState("");
  const [date, setDate] = useState<Date | null>(null);
  const [page, setPage] = useState(1);

  const { transactions, totalPages, loading } = useLostInventory(
    search,
    date ? date.toISOString().split("T")[0] : null, // Convert to YYYY-MM-DD format
    page,
  );

  return (
    <Box
      sx={{
        width: "100%",
        px: { xl: 50 },
      }}
    >
      <Box
        sx={{
          display: "flex",
          width: "80%",
          mx: "auto",
          justifyContent: "space-between",
          alignItems: "center",
          mb: 3,
        }}
      >
        <Typography
          variant="h4"
          component="h1"
          sx={{ color: "text.primary", fontWeight: "bold" }}
        >
          Lost Inventory
        </Typography>
      </Box>

      <LostInventoryFilters
        search={search}
        date={date}
        onSearchChange={(newSearch) => {
          setSearch(newSearch);
          setPage(1);
        }}
        onDateChange={(newDate) => {
          setDate(newDate);
          setPage(1);
        }}
      />

      {loading ? (
        <Box
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            py: 6,
          }}
        >
          <CircularProgress />
        </Box>
      ) : (
        <>
          <LostInventoryTable
            transactions={transactions}
            onTransactionClick={(transaction) => {
              // Optional: Handle transaction click if you want to show details
              console.log("Transaction clicked:", transaction);
            }}
          />

          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              mt: 3,
              "@media (min-width: 1536px)": { mt: 12 },
            }}
          >
            <Pagination
              count={totalPages}
              page={page}
              onChange={(_, v) => setPage(v)}
            />
          </Box>
        </>
      )}

      {!loading && transactions.length === 0 && (
        <Box
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            py: 8,
          }}
        >
          <Typography variant="body1" color="text.secondary">
            No inventory transactions found.
          </Typography>
        </Box>
      )}
    </Box>
  );
}
