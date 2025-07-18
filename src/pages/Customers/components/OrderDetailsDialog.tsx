import Box from "@mui/material/Box";
import CircularProgress from "@mui/material/CircularProgress";
import Typography from "@mui/material/Typography";
import TableDialogLayout from "../../../components/TableDialogLayout";
import useOrderDetails from "../../../hooks/useOrderDetails";
import OrderDetailsTable from "./OrderDetailsTable";

interface Props {
  open: boolean;
  orderId: number;
  onClose: () => void;
}

export default function OrderDetailsDialog({ open, orderId, onClose }: Props) {
  const { details, loading } = useOrderDetails(orderId);

  const handleClose = () => {
    onClose();
  };

  return (
    <TableDialogLayout
      open={open}
      title={`Order Details - #${orderId}`}
      onClose={handleClose}
      onSubmit={handleClose}
      submitText="Close"
    >
      <Box sx={{ display: "flex", flexDirection: "column", height: "100%" }}>
        {loading ? (
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
              flexGrow: 1,
            }}
          >
            <CircularProgress size={24} />
          </Box>
        ) : (
          <Box sx={{ flexGrow: 1, minHeight: 0 }}>
            <OrderDetailsTable details={details} isDialog={true} />
          </Box>
        )}

        {!loading && details.length === 0 && (
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
              flexGrow: 1,
            }}
          >
            <Typography variant="body2" color="text.secondary">
              No details found for this order.
            </Typography>
          </Box>
        )}
      </Box>
    </TableDialogLayout>
  );
}
