import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Button from "@mui/material/Button";
import { LocalizationProvider, DatePicker } from "@mui/x-date-pickers";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFns";
import AppButton from "../../components/AppButton";
import AppSnackbar from "../../components/AppSnackbar";

export default function Reports() {
  const [open, setOpen] = useState(false);
  const [dateReport, setDateReport] = useState("");
  const [startDate, setStartDate] = useState<Date | null>(null);
  const [endDate, setEndDate] = useState<Date | null>(null);
  const [snackOpen, setSnackOpen] = useState(false);
  const [snackMsg, setSnackMsg] = useState("");
  return (
    <>
      <Typography
        variant="h4"
        component="h1"
        className="font-bold"
        sx={{ color: "text.primary", textAlign: "center", mt: 2 }}
      >
        Reports
      </Typography>
      <Box
        sx={{
          minHeight: "80vh",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Box
          sx={{
            display: "grid",
            gridTemplateColumns: "repeat(2, 1fr)",
            gap: 3,
          }}
        >
          <AppButton
            text={"Product Inventory"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {
              await invoke("print_prod_inv_rpt", {
                printerName: localStorage.getItem("fullpage_printer") ?? "",
              });
            }}
          />
          <AppButton
            text={"Customer Balances"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {
              await invoke("print_cust_bal_rpt", {
                printerName: localStorage.getItem("fullpage_printer") ?? "",
              });
            }}
          />
          <AppButton
            text={"Product Catalog"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {
              await invoke("print_product_catalog", {
                printerName: localStorage.getItem("fullpage_printer") ?? "",
              });
            }}
          />
          <AppButton
            text={"Orders By Date"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {
              setDateReport("bycustomer");
              setOpen(true);
            }}
          />
          <AppButton
            text={"Sales Summary By Date"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {
              setDateReport("byproduct");
              setOpen(true);
            }}
          />
          <AppButton
            text={"Amet"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Consectetur"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Adipiscing"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Elit"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Sed"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
        </Box>
      </Box>
      {/* Date‐picker Dialog */}
      <Dialog open={open} onClose={() => setOpen(false)}>
        <DialogTitle>Choose Report Dates</DialogTitle>
        <DialogContent>
          <LocalizationProvider dateAdapter={AdapterDateFns}>
            <DatePicker
              label="Start Date"
              value={startDate}
              onChange={setStartDate}
              slotProps={{
                textField: { margin: "dense", fullWidth: true },
              }}
            />
            <DatePicker
              label="End Date"
              value={endDate}
              onChange={setEndDate}
              slotProps={{
                textField: { margin: "dense", fullWidth: true },
              }}
            />
          </LocalizationProvider>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button
            onClick={async () => {
              if (!startDate || !endDate) {
                setSnackMsg("Please select both start and end dates.");
                setSnackOpen(true);
                return;
              }
              try {
                if (dateReport === "bycustomer") {
                  await invoke("print_sales_detail_report", {
                    startDate: startDate.toISOString(),
                    endDate: endDate.toISOString(),
                    printerName: localStorage.getItem("fullpage_printer") ?? "",
                  });
                } else if (dateReport === "byproduct") {
                  await invoke("print_product_sales_by_category", {
                    startDate: startDate.toISOString(),
                    endDate: endDate.toISOString(),
                    printerName: localStorage.getItem("fullpage_printer") ?? "",
                  });
                }
                setOpen(false);
              } catch (e) {
                setSnackMsg(`Failed: ${e}`);
                setSnackOpen(true);
              }
            }}
          >
            Run
          </Button>
        </DialogActions>
      </Dialog>

      {/* Snackbar for validation/errors */}
      <AppSnackbar
        open={snackOpen}
        message={snackMsg}
        severity="warning"
        onClose={() => setSnackOpen(false)}
      />
    </>
  );
}
