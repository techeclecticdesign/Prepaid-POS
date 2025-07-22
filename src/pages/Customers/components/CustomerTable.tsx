import { useState } from "react";
import Box from "@mui/material/Box";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import CustomerTransactionDialog from "./CustomerTransactionDialog";
import { formatDate, formatCurrency } from "../../../lib/util";
import type { CustomerSearchRow } from "../hooks/useCustomers";

interface Props {
  customers: CustomerSearchRow[];
  onCustomerClick?: (row: CustomerSearchRow) => void;
}

export default function CustomerTable({ customers, onCustomerClick }: Props) {
  const theme = useTheme();
  const [selectedCustomer, setSelectedCustomer] =
    useState<CustomerSearchRow | null>(null);

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

  const handleCustomerClick = (row: CustomerSearchRow) => {
    setSelectedCustomer(row);
    onCustomerClick?.(row);
  };

  return (
    <>
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
            gridTemplateColumns: "10% 35% 15% 20% 20%",
            borderBottom: `1px solid ${theme.palette.divider}`,
            padding: {
              xs: "8px 10px",
              xl: "12px 18px",
            },
          }}
        >
          <Typography variant="caption" sx={headerSx}>
            ID
          </Typography>
          <Typography variant="caption" sx={headerSx}>
            Name
          </Typography>
          <Typography
            variant="caption"
            sx={{ ...headerSx, textAlign: "right" }}
          >
            Balance
          </Typography>
          <Typography
            variant="caption"
            sx={{ ...headerSx, textAlign: "right" }}
          >
            Added
          </Typography>
          <Typography
            variant="caption"
            sx={{ ...headerSx, textAlign: "right" }}
          >
            Updated
          </Typography>
        </Box>

        {/* Body */}
        {customers.map((row, index) => (
          <Box
            key={row.customer.mdoc}
            onClick={() => handleCustomerClick(row)}
            sx={{
              display: "grid",
              gridTemplateColumns: "10% 35% 15% 20% 20%",
              alignItems: "center",
              padding: {
                xs: "8px 10px",
                xl: "12px 18px",
              },
              borderBottom:
                index < customers.length - 1
                  ? `1px solid ${theme.palette.divider}`
                  : "none",
              backgroundColor: "background.paper",
              cursor: "pointer",
              "&:hover": {
                backgroundColor: theme.palette.action.hover,
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
              {row.customer.mdoc}
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
              {row.customer.name}
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
              {formatCurrency(row.balance)}
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
              {formatDate(row.customer.added)}
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
              {formatDate(row.customer.updated)}
            </Typography>
          </Box>
        ))}
      </Box>
      {selectedCustomer && (
        <CustomerTransactionDialog
          open={!!selectedCustomer}
          customerMdoc={selectedCustomer.customer.mdoc}
          customerName={selectedCustomer.customer.name}
          onClose={() => setSelectedCustomer(null)}
        />
      )}
    </>
  );
}
