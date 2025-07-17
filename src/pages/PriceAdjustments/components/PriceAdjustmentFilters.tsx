import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import { DatePicker } from "@mui/x-date-pickers/DatePicker";
import { LocalizationProvider } from "@mui/x-date-pickers/LocalizationProvider";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFns";

interface Props {
  search: string;
  date: Date | null;
  onSearchChange: (search: string) => void;
  onDateChange: (date: Date | null) => void;
}

export default function PriceAdjustmentFilters({
  search,
  date,
  onSearchChange,
  onDateChange,
}: Props) {
  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <Box className="flex flex-col sm:flex-row gap-4 mb-6">
        <TextField
          label="Search adjustments"
          value={search}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder="Search by product name, UPC, or operator..."
          fullWidth
          sx={{ flex: 1 }}
        />
        <DatePicker
          label="Filter by date"
          value={date}
          onChange={onDateChange}
          slotProps={{
            textField: {
              fullWidth: true,
              sx: { flex: 1, maxWidth: { sm: 300 } },
            },
          }}
        />
      </Box>
    </LocalizationProvider>
  );
}
