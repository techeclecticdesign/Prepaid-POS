import { useState } from "react";
import { Pagination } from "@mui/material";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import CircularProgress from "@mui/material/CircularProgress";
import useClubTransactions from "./hooks/useClubTransactions";
import AccountsFilters from "./components/AccountsFilters";
import AccountsTable from "./components/AccountsTable";

export default function AccountsPage() {
  const [search, setSearch] = useState("");
  const [date, setDate] = useState<Date | null>(null);
  const [page, setPage] = useState(1);

  const { transactions, totalPages, loading } = useClubTransactions(
    search,
    date ? date.toISOString().split("T")[0] : null, // Convert to YYYY-MM-DD format
    page,
  );

  return (
    <Box className="p-2 w-4/5 mx-auto 2xl:px-50">
      <Box className="flex justify-between items-center mb-6">
        <Typography
          variant="h4"
          component="h1"
          className="font-bold"
          sx={{ color: "text.primary" }}
        >
          Accounts
        </Typography>
      </Box>

      <AccountsFilters
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
        <Box className="flex justify-center items-center py-8">
          <CircularProgress />
        </Box>
      ) : (
        <>
          <AccountsTable
            transactions={transactions}
            onTransactionClick={(transaction) => {
              console.log("Account transaction clicked:", transaction);
            }}
          />

          <Box className="flex justify-center mt-4 2xl:mt-12">
            <Pagination
              count={totalPages}
              page={page}
              onChange={(_, v) => setPage(v)}
            />
          </Box>
        </>
      )}

      {!loading && transactions.length === 0 && (
        <Box className="flex justify-center items-center py-8">
          <Typography variant="body1" color="text.secondary">
            No account transactions found.
          </Typography>
        </Box>
      )}
    </Box>
  );
}
