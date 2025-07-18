import { useState } from "react";
import Box from "@mui/material/Box";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import OrderDetailsDialog from "./OrderDetailsDialog";
import type { CustomerTransactionSearchRow } from "../../../hooks/useCustomerTransactions";

interface Props {
  transactions: CustomerTransactionSearchRow[];
  onTransactionClick?: (transaction: CustomerTransactionSearchRow) => void;
  isDialog?: boolean;
}

export default function CustomerTransactionTable({
  transactions,
  onTransactionClick,
  isDialog = false,
}: Props) {
  const theme = useTheme();

  const [selectedOrderId, setSelectedOrderId] = useState<number | null>(null);

  const headerSx = {
    color: "text.secondary",
    fontWeight: theme.typography.fontWeightMedium as number,
    textTransform: "uppercase",
    letterSpacing: "0.05em",
    fontSize: {
      xs: theme.typography.pxToRem(isDialog ? 8 : 10),
      xl: theme.typography.pxToRem(isDialog ? 14 : 16),
    },
  };

  const cellTextSx = {
    color: "text.primary",
    fontSize: {
      xs: theme.typography.pxToRem(isDialog ? 10 : 12),
      xl: theme.typography.pxToRem(isDialog ? 15 : 18),
    },
  };

  const paddingSx = {
    xs: isDialog ? "4px 6px" : "8px 10px",
    xl: isDialog ? "14px 18px" : "14px 26px",
  };

  const formatDate = (dateString: string | null | undefined) => {
    if (!dateString) {
      return "-";
    }
    return new Date(dateString).toLocaleDateString();
  };

  const formatCurrency = (cents: number) => {
    return `$${(cents / 100).toFixed(2)}`;
  };

  return (
    <Box
      sx={{
        border: `1px solid ${theme.palette.divider}`,
        borderRadius: 1,
        overflow: "hidden",
        height: isDialog ? "100%" : "auto",
        mr: isDialog ? 1 : 4,
        display: "flex",
        flexDirection: "column",
      }}
    >
      {/* Header */}
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "15% 18% 18% 18% 18% 13%", // Date, Order ID, Operator, Note
          borderBottom: `1px solid ${theme.palette.divider}`,
          padding: paddingSx,
          flexShrink: 0, // Prevent header from shrinking
        }}
      >
        <Typography variant="caption" sx={headerSx}>
          Date
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Order ID
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Operator
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Spent
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Note
        </Typography>
      </Box>

      {/* Body */}
      <Box
        sx={{
          flexGrow: 1,
        }}
      >
        {transactions.map((row, index) => (
          <Box
            key={row.transaction.order_id}
            onClick={() => {
              setSelectedOrderId(row.transaction.order_id);
              onTransactionClick?.(row);
            }}
            sx={{
              display: "grid",
              gridTemplateColumns: "15% 18% 18% 18% 18% 13%",
              alignItems: "center",
              padding: paddingSx,
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
              {row.transaction.order_id}
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
            <Typography
              variant="body2"
              sx={{
                ...cellTextSx,
                overflow: "hidden",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
              }}
            >
              {formatCurrency(row.spent)}
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
              {row.transaction.note || "-"}
            </Typography>
          </Box>
        ))}
      </Box>
      {selectedOrderId && (
        <OrderDetailsDialog
          open={!!selectedOrderId}
          orderId={selectedOrderId}
          onClose={() => setSelectedOrderId(null)}
        />
      )}
    </Box>
  );
}
