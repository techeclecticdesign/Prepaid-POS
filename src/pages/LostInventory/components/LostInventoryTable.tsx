import Box from "@mui/material/Box";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import Chip from "@mui/material/Chip";

interface Transaction {
  id: number | null;
  upc: string;
  quantity_change: number;
  reference: string | null | undefined;
  created_at: string | null | undefined;
}

interface Row {
  product_name: string;
  transaction: Transaction;
  operator_name: string;
}

interface Props {
  transactions: Row[];
  onTransactionClick?: (row: Row) => void;
}

export default function LostInventoryTable({
  transactions,
  onTransactionClick,
}: Props) {
  const theme = useTheme();

  const headerSx = {
    color: "text.secondary",
    fontWeight: theme.typography.fontWeightMedium as number,
    textTransform: "uppercase",
    letterSpacing: "0.05em",
    fontSize: {
      xs: theme.typography.pxToRem(10),
      xl: theme.typography.pxToRem(13),
    },
  };

  const cellTextSx = {
    color: "text.primary",
    fontSize: {
      xs: theme.typography.pxToRem(12),
      xl: theme.typography.pxToRem(14),
    },
  };

  const formatDate = (dateString: string | null | undefined) => {
    if (!dateString) {
      return "-";
    }
    return new Date(dateString).toLocaleDateString();
  };

  const getQuantityChangeColor = (change: number) => {
    if (change > 0) return "success";
    if (change < 0) return "error";
    return "default";
  };

  return (
    <Box
      sx={{
        border: `1px solid ${theme.palette.divider}`,
        borderRadius: 1,
        overflow: "hidden",
        mx: "auto",
        width: "80%",
      }}
    >
      {/* Header */}
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "30% 20% 10% 25% 15%",
          borderBottom: `1px solid ${theme.palette.divider}`,
          padding: {
            xs: "6px 8px",
            xl: "10px 15px",
          },
        }}
      >
        <Typography variant="caption" sx={headerSx}>
          Product
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Operator
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "center" }}>
          Change
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Reference
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "right" }}>
          Date
        </Typography>
      </Box>

      {/* Body */}
      {transactions.map((row, index) => (
        <Box
          key={row.transaction.id ?? index}
          onClick={() => onTransactionClick?.(row)}
          sx={{
            display: "grid",
            gridTemplateColumns: "30% 20% 10% 25% 15%",
            alignItems: "center",
            padding: {
              xs: "6px 8px",
              xl: "10px 15px",
            },
            borderBottom:
              index < transactions.length - 1
                ? `1px solid ${theme.palette.divider}`
                : "none",
            backgroundColor: "background.paper",
            cursor: onTransactionClick ? "pointer" : "default",
            "&:hover": {
              backgroundColor: onTransactionClick
                ? theme.palette.action.hover
                : "transparent",
            },
          }}
        >
          <Typography
            variant="body2"
            sx={{
              ...cellTextSx,
              overflow: "hidden",
              textOverflow: "ellipsis",
              whiteSpace: "nowrap",
            }}
          >
            {row.product_name}
          </Typography>
          <Typography
            variant="body2"
            sx={{
              ...cellTextSx,
              overflow: "hidden",
              textOverflow: "ellipsis",
              whiteSpace: "nowrap",
            }}
          >
            {row.operator_name}
          </Typography>
          <Box sx={{ display: "flex", justifyContent: "center" }}>
            <Chip
              label={
                row.transaction.quantity_change > 0
                  ? `+${row.transaction.quantity_change}`
                  : row.transaction.quantity_change
              }
              color={getQuantityChangeColor(row.transaction.quantity_change)}
              size="small"
            />
          </Box>
          <Typography
            variant="body2"
            sx={{
              ...cellTextSx,
              overflow: "hidden",
              textOverflow: "ellipsis",
              whiteSpace: "nowrap",
            }}
          >
            {row.transaction.reference || "-"}
          </Typography>
          <Typography
            variant="body2"
            sx={{
              ...cellTextSx,
              textAlign: "right",
              overflow: "hidden",
              textOverflow: "ellipsis",
              whiteSpace: "nowrap",
            }}
          >
            {formatDate(row.transaction.created_at)}
          </Typography>
        </Box>
      ))}
    </Box>
  );
}
