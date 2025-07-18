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
    <Box className="p-2 w-4/5 mx-auto 2xl:px-50">
      <Box className="flex justify-between items-center mb-6">
        <Typography
          variant="h4"
          component="h1"
          className="font-bold"
          sx={{ color: "text.primary" }}
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
        <Box className="flex justify-center items-center py-8">
          <CircularProgress />
        </Box>
      ) : (
        <>
          <CustomerTable customers={customers} />

          <Box className="flex justify-center mt-4 2xl:mt-12">
            <Pagination
              count={totalPages}
              page={page}
              onChange={(_, v) => setPage(v)}
            />
          </Box>
        </>
      )}

      {!loading && customers.length === 0 && (
        <Box className="flex justify-center items-center py-8">
          <Typography variant="body1" color="text.secondary">
            No customers found.
          </Typography>
        </Box>
      )}
    </Box>
  );
}
