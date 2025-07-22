import Box from "@mui/material/Box";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import { formatDate, formatCurrency } from "../../../lib/util";
import type { ClubTransactionSearchRow } from "../hooks/useClubTransactions";

interface Props {
  transactions: ClubTransactionSearchRow[];
  onTransactionClick?: (row: ClubTransactionSearchRow) => void;
}

export default function AccountsTable({
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

  return (
    <Box
      sx={{
        border: `1px solid ${theme.palette.divider}`,
        borderRadius: 1,
        overflow: "hidden",
        mr: 4,
      }}
    >
      {/* Header */}
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "25% 35% 20% 20%", // Date, Name, Mdoc, Amount
          borderBottom: `1px solid ${theme.palette.divider}`,
          padding: {
            xs: "8px 10px",
            xl: "12px 18px",
          },
        }}
      >
        <Typography variant="caption" sx={headerSx}>
          Date
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Name
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Mdoc
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "right" }}>
          Amount
        </Typography>
      </Box>

      {/* Body */}
      {transactions.map((row, index) => (
        <Box
          key={row.transaction.id}
          onClick={() => onTransactionClick?.(row)}
          sx={{
            display: "grid",
            gridTemplateColumns: "25% 35% 20% 20%", // Date, Name, Mdoc, Amount
            alignItems: "center",
            padding: {
              xs: "8px 10px",
              xl: "12px 18px",
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
            {formatDate(row.transaction.date)}
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
            {row.customer_name}
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
            {row.transaction.mdoc}
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
            {formatCurrency(row.transaction.amount)}
          </Typography>
        </Box>
      ))}
    </Box>
  );
}
