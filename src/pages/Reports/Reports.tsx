import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import AppButton from "../../components/AppButton";
import AppSnackbar from "../../components/AppSnackbar";
import ClubImportSelectModal from "./components/ClubImportSelect";
import DateRangeDialog from "./components/DateRangeDialog";

export default function Reports() {
  const [open, setOpen] = useState(false);
  const [dateReport, setDateReport] = useState("");
  const [snackOpen, setSnackOpen] = useState(false);
  const [snackMsg, setSnackMsg] = useState("");
  const [importModalOpen, setImportModalOpen] = useState(false);
  return (
    <>
      <Typography
        variant="h4"
        component="h1"
        sx={{
          fontWeight: "bold",
          color: "text.primary",
          textAlign: "center",
          mt: 2,
        }}
      >
        Reports
      </Typography>
      <Box
        sx={{
          minHeight: "70vh",
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
                sumatraLocation: localStorage.getItem("sumatra_path") ?? "",
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
                sumatraLocation: localStorage.getItem("sumatra_path") ?? "",
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
                sumatraLocation: localStorage.getItem("sumatra_path") ?? "",
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
            text={"Sales By Day"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {
              setDateReport("byday");
              setOpen(true);
            }}
          />
          <AppButton
            text={"Club Import"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={() => setImportModalOpen(true)}
          />
          <AppButton
            text={"Adipiscing"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
        </Box>
      </Box>
      <DateRangeDialog
        open={open}
        dateReport={dateReport}
        onClose={() => setOpen(false)}
        setSnackOpen={setSnackOpen}
        setSnackMsg={setSnackMsg}
      />
      <ClubImportSelectModal
        open={importModalOpen}
        onClose={() => setImportModalOpen(false)}
      />
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
