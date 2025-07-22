import Box from "@mui/material/Box";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import Chip from "@mui/material/Chip";
import { formatDate, formatCurrency } from "../../../lib/util";
import type { PriceAdjustmentSearchRow } from "../hooks/usePriceAdjustments";

interface Props {
  adjustments: PriceAdjustmentSearchRow[];
  onAdjustmentClick?: (row: PriceAdjustmentSearchRow) => void;
}

export default function PriceAdjustmentTable({
  adjustments,
  onAdjustmentClick,
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

  const getPriceChangeColor = (oldPrice: number, newPrice: number) => {
    if (newPrice > oldPrice) return "success";
    if (newPrice < oldPrice) return "error";
    return "default";
  };

  const getPriceChangeLabel = (oldPrice: number, newPrice: number) => {
    const change = newPrice - oldPrice;
    if (change > 0) return `+${formatCurrency(change)}`;
    if (change < 0) return formatCurrency(change);
    return formatCurrency(0);
  };

  console.log("adjustments", adjustments);

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
          gridTemplateColumns: "33% 21% 12% 12% 12% 10%",
          borderBottom: `1px solid ${theme.palette.divider}`,
          padding: {
            xs: "8px 10px",
            xl: "12px 18px",
          },
        }}
      >
        <Typography variant="caption" sx={headerSx}>
          Product
        </Typography>
        <Typography variant="caption" sx={headerSx}>
          Operator
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "right" }}>
          Old Price
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "right" }}>
          New Price
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "center" }}>
          Change
        </Typography>
        <Typography variant="caption" sx={{ ...headerSx, textAlign: "right" }}>
          Date
        </Typography>
      </Box>

      {/* Body */}
      {adjustments.map((row, index) => (
        <Box
          key={row.adjustment.id}
          onClick={() => onAdjustmentClick?.(row)}
          sx={{
            display: "grid",
            gridTemplateColumns: "33% 21% 12% 12% 12% 10%",
            alignItems: "center",
            padding: {
              xs: "8px 10px",
              xl: "12px 18px",
            },
            borderBottom:
              index < adjustments.length - 1
                ? `1px solid ${theme.palette.divider}`
                : "none",
            backgroundColor: "background.paper",
            cursor: onAdjustmentClick ? "pointer" : "default",
            "&:hover": {
              backgroundColor: onAdjustmentClick
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
            {formatCurrency(row.adjustment.old)}
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
            {formatCurrency(row.adjustment.new)}
          </Typography>
          <Box sx={{ display: "flex", justifyContent: "center" }}>
            <Chip
              label={getPriceChangeLabel(
                row.adjustment.old,
                row.adjustment.new,
              )}
              color={getPriceChangeColor(
                row.adjustment.old,
                row.adjustment.new,
              )}
              size="small"
            />
          </Box>
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
            {formatDate(row.adjustment.created_at)}
          </Typography>
        </Box>
      ))}
    </Box>
  );
}
