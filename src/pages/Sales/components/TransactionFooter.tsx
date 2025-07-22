import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import { useTheme } from "@mui/material/styles";
import { formatCurrency } from "../../../lib/util";
import type { CustomerPosDto } from "../hooks/usePosInit";

interface Props {
  selectedCustomer: CustomerPosDto | null;
  transactionTotal: number;
  sessionSales: number;
  sessionCustomers: number;
  onAcceptReturn: () => void;
  onCancelTransaction: () => void;
  onSubmitTransaction: () => void;
}

export default function TransactionFooter({
  selectedCustomer,
  transactionTotal,
  sessionSales,
  sessionCustomers,
  onAcceptReturn,
  onCancelTransaction,
  onSubmitTransaction,
}: Props) {
  const theme = useTheme();

  const remainingBalance = selectedCustomer
    ? selectedCustomer.balance - transactionTotal
    : 0;

  const footerSx = {
    position: "fixed",
    bottom: 0,
    left: 0,
    right: "20rem",
    backgroundColor: theme.palette.background.paper,
    borderTop: `1px solid ${theme.palette.divider}`,
    p: 2,
    zIndex: theme.zIndex.appBar,
    boxShadow: theme.shadows[4],
  };

  const columnSx = {
    display: "flex",
    flexDirection: "column",
    gap: 0.5,
    flex: 1,
  };

  const labelSx = {
    fontSize: {
      xs: theme.typography.pxToRem(12),
      xl: theme.typography.pxToRem(14),
    },
    fontWeight: theme.typography.fontWeightMedium,
    color: "text.primary",
  };

  const valueSx = {
    fontSize: {
      xs: theme.typography.pxToRem(14),
      xl: theme.typography.pxToRem(16),
    },
    fontWeight: theme.typography.fontWeightBold,
    color: "text.primary",
  };

  const buttonSx = {
    fontSize: {
      xs: theme.typography.pxToRem(10),
      md: theme.typography.pxToRem(12),
    },
    minHeight: 32,
    mb: 0.5,
  };

  return (
    <Box sx={footerSx}>
      <Box sx={{ display: "flex", gap: 3, alignItems: "stretch" }}>
        {/* Left Column - Customer Financial Info */}
        <Box sx={columnSx}>
          <Box>
            <Typography variant="body2" sx={labelSx}>
              Total:
            </Typography>
            <Typography variant="body1" sx={valueSx}>
              {selectedCustomer ? formatCurrency(transactionTotal) : "-"}
            </Typography>
          </Box>
          <Box>
            <Typography variant="body2" sx={labelSx}>
              Account Balance:
            </Typography>
            <Typography variant="body1" sx={valueSx}>
              {selectedCustomer
                ? formatCurrency(selectedCustomer.balance)
                : "-"}
            </Typography>
          </Box>
          <Box>
            <Typography variant="body2" sx={labelSx}>
              Remaining Balance:
            </Typography>
            <Typography
              variant="body1"
              sx={{
                ...valueSx,
                color:
                  remainingBalance < 0
                    ? theme.palette.error.main
                    : "text.primary",
              }}
            >
              {selectedCustomer ? formatCurrency(remainingBalance) : "-"}
            </Typography>
          </Box>
        </Box>

        {/* Middle Column - Session Stats */}
        <Box sx={columnSx}>
          <Box>
            <Typography variant="body2" sx={labelSx}>
              Amount sold this session:
            </Typography>
            <Typography variant="body1" sx={valueSx}>
              {formatCurrency(sessionSales)}
            </Typography>
          </Box>
          <Box>
            <Typography variant="body2" sx={labelSx}>
              Customers this session:
            </Typography>
            <Typography variant="body1" sx={valueSx}>
              {sessionCustomers}
            </Typography>
          </Box>
        </Box>

        {/* Right Column - Action Buttons */}
        <Box sx={{ ...columnSx, minWidth: 150 }}>
          <Button
            variant="outlined"
            color="warning"
            sx={buttonSx}
            onClick={onAcceptReturn}
          >
            Accept Return
          </Button>
          <Button
            variant="outlined"
            color="error"
            sx={buttonSx}
            onClick={onCancelTransaction}
          >
            Cancel Transaction
          </Button>
          <Button
            variant="contained"
            color="primary"
            sx={buttonSx}
            onClick={onSubmitTransaction}
            disabled={!selectedCustomer || transactionTotal === 0}
          >
            Submit Transaction
          </Button>
        </Box>
      </Box>
    </Box>
  );
}
