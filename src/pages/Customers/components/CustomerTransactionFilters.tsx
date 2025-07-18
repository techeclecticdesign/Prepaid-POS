import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import { DatePicker } from "@mui/x-date-pickers/DatePicker";
import { LocalizationProvider } from "@mui/x-date-pickers/LocalizationProvider";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFns";

interface Props {
  search?: string;
  date: Date | null;
  onSearchChange?: (search: string) => void;
  onDateChange: (date: Date | null) => void;
  isDialog?: boolean;
}

export default function CustomerTransactionFilters({
  search = "",
  date,
  onSearchChange,
  onDateChange,
  isDialog = false,
}: Props) {
  const containerSx = {
    mb: isDialog ? 2 : 4,
    gap: isDialog ? 2 : 4,
  };

  const textFieldSx = {
    flex: 1,
    "& .MuiInputBase-root": {
      fontSize: isDialog ? "0.875rem" : "1rem",
    },
    "& .MuiInputLabel-root": {
      fontSize: isDialog ? "0.875rem" : "1rem",
    },
  };

  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <Box className="flex flex-col sm:flex-row" sx={containerSx}>
        {onSearchChange && (
          <TextField
            label="Search transactions"
            value={search}
            onChange={(e) => onSearchChange(e.target.value)}
            placeholder="Search by order ID or note..."
            fullWidth
            sx={textFieldSx}
          />
        )}
        <DatePicker
          label="Filter by date"
          value={date}
          onChange={onDateChange}
          slotProps={{
            textField: {
              fullWidth: true,
              size: isDialog ? "small" : "medium",
              sx: {
                ...textFieldSx,
                maxWidth: { sm: isDialog ? 250 : 300 },
              },
            },
          }}
        />
      </Box>
    </LocalizationProvider>
  );
}
