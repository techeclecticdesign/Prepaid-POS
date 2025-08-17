import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";

interface Props {
  search: string;
  onSearchChange: (search: string) => void;
}

export default function CustomerFilters({ search, onSearchChange }: Props) {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: { xs: "column", sm: "row" },
        gap: 4,
        mb: 6,
      }}
    >
      <TextField
        label="Search customers"
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
        placeholder="Search by customer name..."
        fullWidth
        sx={{ flex: 1, maxWidth: "300px" }}
      />
    </Box>
  );
}
