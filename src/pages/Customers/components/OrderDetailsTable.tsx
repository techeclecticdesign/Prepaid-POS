import Box from "@mui/material/Box";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import { formatCurrency } from "../../../lib/util";
import type { CustomerTxDetailDto } from "../../../hooks/useOrderDetails";

interface Props {
  details: CustomerTxDetailDto[];
  onDetailClick?: (detail: CustomerTxDetailDto) => void;
  isDialog?: boolean;
}

export default function OrderDetailsTable({
  details,
  onDetailClick,
  isDialog = false,
}: Props) {
  const theme = useTheme();

  // Adjust sizes based on dialog layout
  const headerSx = {
    color: "text.secondary",
    fontWeight: theme.typography.fontWeightMedium as number,
    textTransform: "uppercase",
    letterSpacing: "0.05em",
    fontSize: {
      xs: theme.typography.pxToRem(isDialog ? 8 : 10),
      xl: theme.typography.pxToRem(isDialog ? 11 : 13),
    },
  };

  const cellTextSx = {
    color: "text.primary",
    fontSize: {
      xs: theme.typography.pxToRem(isDialog ? 10 : 12),
      xl: theme.typography.pxToRem(isDialog ? 12 : 14),
    },
  };

  const paddingSx = {
    xs: isDialog ? "4px 6px" : "8px 10px",
    xl: isDialog ? "8px 12px" : "12px 18px",
  };

  return (
    <Box
      sx={{
        border: `1px solid ${theme.palette.divider}`,
        borderRadius: 1,
        overflow: "hidden",
        mr: isDialog ? 1 : 4,
        height: isDialog ? "100%" : "auto",
        display: "flex",
        flexDirection: "column",
      }}
    >
      {/* Header */}
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "40% 20% 15% 25%", // Item, UPC, Qty, Price
          borderBottom: `1px solid ${theme.palette.divider}`,
          padding: paddingSx,
          flexShrink: 0,
          backgroundColor: theme.palette.grey[50],
        }}
      >
        <Typography variant="caption" sx={headerSx}>
          Item
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          UPC
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "center" }}>
          Qty
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "right" }}>
          Price
        </Typography>
      </Box>

      {/* Body */}
      <Box
        sx={{
          flexGrow: 1,
          overflowY: isDialog ? "auto" : "visible",
        }}
      >
        {details.map((detail, index) => (
          <Box
            key={detail.detail_id}
            onClick={() => onDetailClick?.(detail)}
            sx={{
              display: "grid",
              gridTemplateColumns: "40% 20% 15% 25%",
              alignItems: "center",
              padding: paddingSx,
              borderBottom:
                index < details.length - 1
                  ? `1px solid ${theme.palette.divider}`
                  : "none",
              backgroundColor: "background.paper",
              cursor: onDetailClick ? "pointer" : "default",
              "&:hover": {
                backgroundColor: onDetailClick
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
              {detail.product_name}
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
              {detail.upc}
            </Typography>
            <Typography
              variant="body2"
              sx={{
                ...cellTextSx,
                textAlign: "center",
                overflow: "hidden",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
              }}
            >
              {detail.quantity}
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
              {formatCurrency(detail.price)}
            </Typography>
          </Box>
        ))}
      </Box>
    </Box>
  );
}
