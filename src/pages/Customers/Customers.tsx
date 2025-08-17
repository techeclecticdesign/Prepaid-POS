import { useState } from "react";
import { Pagination } from "@mui/material";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import CircularProgress from "@mui/material/CircularProgress";
import useCustomers from "./hooks/useCustomers";
import CustomerFilters from "./components/CustomerFilters";
import CustomerTable from "./components/CustomerTable";

export default function CustomersPage() {
  const [search, setSearch] = useState("");
  const [page, setPage] = useState(1);

  const { customers, totalPages, loading } = useCustomers(search, page);

  return (
    <Box sx={{ width: "80%", mx: "auto", px: { xl2: 50 } }}>
      <Box
        sx={{
          display: "flex",
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
          Customers
        </Typography>
      </Box>

      <CustomerFilters
        search={search}
        onSearchChange={(newSearch) => {
          setSearch(newSearch);
          setPage(1);
        }}
      />

      {loading ? (
        <Box
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            py: 4,
          }}
        >
          <CircularProgress />
        </Box>
      ) : (
        <>
          <CustomerTable customers={customers} />

          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              mt: { xs: 4, xl: 6 },
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

      {!loading && customers.length === 0 && (
        <Box
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            py: 8,
          }}
        >
          <Typography variant="body1" color="text.secondary">
            No customers found.
          </Typography>
        </Box>
      )}
    </Box>
  );
}
