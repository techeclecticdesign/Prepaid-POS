import { invoke } from "@tauri-apps/api/core";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import AppButton from "../../components/AppButton";

export default function Reports() {
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
            text={"Lorem"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Ipsum"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Dolor"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Sit"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
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
          <AppButton
            text={"Do"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
          <AppButton
            text={"Eiusmod"}
            variant="outlined"
            sx={{ width: "14rem", height: "3rem" }}
            onClick={async () => {}}
          />
        </Box>
      </Box>
    </>
  );
}
