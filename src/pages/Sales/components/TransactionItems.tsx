import { useEffect, useRef } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import {
  DataGrid,
  type GridColDef,
  type GridRowsProp,
  type GridRowModel,
} from "@mui/x-data-grid";
import { useTheme } from "@mui/material/styles";
import DeleteIcon from "@mui/icons-material/Delete";
import { formatCurrency } from "../../../lib/util";
import type Product from "../../../models/Product";

export interface TransactionItem {
  id: string;
  upc: string;
  name: string;
  price: number; // in cents
  quantity: number;
}

interface Props {
  scannedUpc: string | null;
  products: Product[];
  transactionItems: TransactionItem[];
  setTransactionItems: React.Dispatch<React.SetStateAction<TransactionItem[]>>;
  onTotalChange: (newTotal: number) => void;
  availableBalance: number;
  onInsufficientFunds: () => void;
}

export default function TransactionItems({
  scannedUpc,
  products,
  transactionItems,
  setTransactionItems,
  onTotalChange,
  availableBalance,
  onInsufficientFunds,
}: Props) {
  const theme = useTheme();
  const gridRef = useRef<HTMLDivElement>(null);

  const dataGridSx = {
    "& .MuiDataGrid-cell": {
      fontSize: {
        xs: theme.typography.pxToRem(12),
        xl: theme.typography.pxToRem(24),
      },
      backgroundColor: "background.paper",
    },
    "& .MuiDataGrid-columnHeaders": {
      minHeight: 32,
      maxHeight: 32,
    },
    "& .MuiDataGrid-columnHeader": {
      minHeight: 32,
      maxHeight: 32,
      lineHeight: "32px",
      fontSize: {
        xs: theme.typography.pxToRem(12),
        xl: theme.typography.pxToRem(24),
      },
      fontWeight: theme.typography.fontWeightMedium,
    },
    "& .MuiDataGrid-columnHeaderTitle": {
      fontSize: "0.75rem",
      lineHeight: "32px",
      whiteSpace: "nowrap",
      overflow: "hidden",
      textOverflow: "ellipsis",
    },
  };

  /*
   *  Creates virtual cart with incoming changes added to current cart.  Uses this to check
   * if customer has funds to cover the new changes and notifies if not.
   */
  const getScannedCart = (
    prevItems: TransactionItem[],
    upc: string,
  ): TransactionItem[] => {
    // Find existing
    const idx = prevItems.findIndex((i) => i.upc === upc);
    if (idx >= 0) {
      // bump quantity
      return prevItems.map((it, i) =>
        i === idx ? { ...it, quantity: it.quantity + 1 } : it,
      );
    }
    // add new line
    const p = products.find((x) => x.upc === upc);
    return p
      ? [
          ...prevItems,
          {
            id: p.upc,
            upc: p.upc,
            name: p.desc,
            price: p.price,
            quantity: 1,
          },
        ]
      : prevItems;
  };

  // Handle new scanned UPC
  useEffect(() => {
    if (!scannedUpc) return;

    setTransactionItems((prevItems) => {
      // build the would-be cart
      const updatedItems = getScannedCart(prevItems, scannedUpc);

      // calc new total and check balance
      const total = updatedItems.reduce(
        (s, it) => s + it.price * it.quantity,
        0,
      );

      if (total > availableBalance) {
        onInsufficientFunds();
        return prevItems;
      }

      onTotalChange(total);
      return updatedItems;
    });
  }, [
    scannedUpc,
    products,
    availableBalance,
    onInsufficientFunds,
    onTotalChange,
    setTransactionItems,
  ]);

  useEffect(() => {
    // Scroll to bottom of DataGrid as items are added to large orders
    if (!gridRef.current) return;
    const virtualScroller = gridRef.current.querySelector(
      ".MuiDataGrid-virtualScroller",
    ) as HTMLDivElement | null;
    if (virtualScroller) {
      virtualScroller.scrollTop = virtualScroller.scrollHeight;
    }
  }, [transactionItems]);

  // remove a single item by id
  const handleDelete = (id: string) => {
    setTransactionItems((prev) => prev.filter((item) => item.id !== id));
    // recalc total after removal
    const newItems = transactionItems.filter((item) => item.id !== id);
    onTotalChange(newItems.reduce((sum, i) => sum + i.price * i.quantity, 0));
  };

  // Handle quantity changes from DataGrid
  const handleRowUpdate = (newRow: GridRowModel) => {
    const updatedQuantity = Math.max(0, Number(newRow.quantity) || 0); // Ensure non-negative quantity

    // find the original item
    const orig = transactionItems.find((it) => it.id === newRow.id);
    if (!orig) return { ...newRow, quantity: updatedQuantity };

    // build post-edit cart
    const updatedItems = transactionItems
      .map((it) =>
        it.id === newRow.id ? { ...it, quantity: updatedQuantity } : it,
      )
      .filter((it) => it.quantity > 0);

    // calc new total & block if it exceeds balance
    const total = updatedItems.reduce((s, it) => s + it.price * it.quantity, 0);
    if (total > availableBalance) {
      onInsufficientFunds();
      return { ...newRow, quantity: orig.quantity }; // revert UI
    }
    setTransactionItems(updatedItems);
    onTotalChange(total);
    return { ...newRow, quantity: updatedQuantity };
  };

  const columns: GridColDef[] = [
    {
      field: "name",
      headerName: "Item",
      width: 300,
      editable: false,
    },
    {
      field: "price",
      headerName: "Price",
      width: 100,
      editable: false,
      valueFormatter: (value) => formatCurrency(value),
    },
    {
      field: "quantity",
      headerName: "Qty",
      width: 80,
      editable: true,
      type: "number",
    },
    {
      field: "total",
      headerName: "Total",
      width: 120,
      editable: false,
      valueGetter: (_value, row) => formatCurrency(row.price * row.quantity),
    },
    // column with button to delete row
    {
      field: "actions",
      headerName: "",
      width: 60,
      sortable: false,
      filterable: false,
      renderCell: (params) => (
        <IconButton
          size="small"
          onClick={() => handleDelete(params.id as string)}
          aria-label="delete"
        >
          <DeleteIcon fontSize="small" />
        </IconButton>
      ),
    },
  ];

  const rows: GridRowsProp = transactionItems.map((item) => ({
    id: item.id,
    name: item.name,
    price: item.price,
    quantity: item.quantity,
  }));

  return (
    <Box sx={{ width: "100%", maxWidth: 800, mx: "auto", p: 2, mt: 5 }}>
      {transactionItems.length === 0 ? (
        <Box
          sx={{
            p: 4,
            textAlign: "center",
            border: `1px dashed ${theme.palette.divider}`,
            borderRadius: 1,
          }}
        >
          <Typography variant="body2" color="text.secondary">
            Scan items to add them to the transaction
          </Typography>
        </Box>
      ) : (
        <Box
          ref={gridRef}
          sx={{
            height: "60vh",
            width: "100%",
            minWidth: 300,
          }}
        >
          <DataGrid
            rows={rows}
            columns={columns}
            rowHeight={32}
            processRowUpdate={handleRowUpdate}
            onProcessRowUpdateError={(error) => {
              console.error("Error updating row:", error);
            }}
            hideFooter
            disableRowSelectionOnClick
            sx={dataGridSx}
          />
        </Box>
      )}
    </Box>
  );
}
