import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Typography from "@mui/material/Typography";
import type Product from "../../../models/Product";
import { useTheme } from "@mui/material/styles";

interface Props {
  products: Product[];
  onProductClick: (product: Product) => void;
}

export default function ProductsTable({ products, onProductClick }: Props) {
  const theme = useTheme();

  const headerSx = {
    marginLeft: {
      xs: 0.8,
      xl: 1,
    },
    color: "text.secondary",
    fontWeight: theme.typography.fontWeightMedium as number,
    textTransform: "uppercase",
    letterSpacing: "0.05em",
    fontSize: {
      xs: theme.typography.pxToRem(12),
      xl: theme.typography.pxToRem(16),
    },
  };

  const cellSx = {
    padding: {
      xs: "8px 18px",
      xl: "28px 22px",
    },
  };

  const cellTextSx = {
    color: "text.primary",
    whiteSpace: "nowrap",
    fontSize: {
      xs: theme.typography.pxToRem(14),
      xl: theme.typography.pxToRem(20),
    },
  };

  return (
    <Table
      className="min-w-full"
      sx={{
        "& .MuiTableCell-root": {
          borderColor: "divider",
        },
      }}
    >
      <TableHead>
        <TableRow>
          <TableCell sx={{ paddingBottom: 0, paddingLeft: 1.2 }}>
            <Typography variant="caption" sx={headerSx}>
              Description
            </Typography>
          </TableCell>
          <TableCell sx={{ paddingBottom: 0, paddingLeft: 1.2 }}>
            <Typography variant="body2" sx={headerSx}>
              Category
            </Typography>
          </TableCell>
          <TableCell
            sx={{ textAlign: "right", paddingBottom: 0, paddingRight: 2 }}
          >
            <Typography variant="caption" sx={headerSx}>
              Qty
            </Typography>
          </TableCell>
          <TableCell
            sx={{ textAlign: "right", paddingBottom: 0, paddingRight: 2 }}
          >
            <Typography variant="caption" sx={{ ...headerSx }}>
              Price
            </Typography>
          </TableCell>
        </TableRow>
      </TableHead>
      <TableBody
        sx={{
          backgroundColor: "background.paper",
          "& .MuiTableRow-root:hover": {
            backgroundColor: theme.palette.action.hover,
            cursor: "pointer",
          },
        }}
      >
        {products.map((p) => (
          <TableRow key={p.upc} onClick={() => onProductClick(p)}>
            <TableCell sx={cellSx}>
              <Typography variant="body2" sx={cellTextSx}>
                {p.desc}
              </Typography>
            </TableCell>
            <TableCell sx={cellSx}>
              <Typography variant="body2" sx={cellTextSx}>
                {p.category}
              </Typography>
            </TableCell>
            <TableCell
              sx={{ ...cellSx, textAlign: "right", cursor: "pointer" }}
              onClick={(e) => {
                e.stopPropagation();
              }}
            >
              <Typography variant="body2" sx={{ ...cellTextSx }}>
                {p.available}
              </Typography>
            </TableCell>
            <TableCell sx={{ ...cellSx, textAlign: "right" }}>
              <Typography variant="body2" sx={cellTextSx}>
                ${(p.price / 100).toFixed(2)}
              </Typography>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
