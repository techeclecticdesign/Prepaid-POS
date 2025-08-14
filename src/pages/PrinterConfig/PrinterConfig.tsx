import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useEffect, useState } from "react";
import Box from "@mui/material/Box";
import AppButton from "../../components/AppButton";
import Typography from "@mui/material/Typography";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select, { SelectChangeEvent } from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import FormHelperText from "@mui/material/FormHelperText";
import CircularProgress from "@mui/material/CircularProgress";
import Backdrop from "@mui/material/Backdrop";

interface PrinterDto {
  name: string;
}

export default function PrinterConfigPage() {
  const [sumatraPath, setSumatraPath] = useState<string>(
    () => localStorage.getItem("sumatra_path") || "",
  );
  const [printers, setPrinters] = useState<PrinterDto[]>([]);
  const [receiptPrinter, setReceiptPrinter] = useState<string>(
    () => localStorage.getItem("receipt_printer") || "",
  );
  const [fullPagePrinter, setFullPagePrinter] = useState<string>(
    () => localStorage.getItem("fullpage_printer") || "",
  );
  const [loading, setLoading] = useState(true);

  // Fetch printers once on mount
  useEffect(() => {
    invoke<PrinterDto[]>("list_printers")
      .then((list) => setPrinters(list))
      .catch((e) => {
        console.error("Failed to list printers:", e);
        setPrinters([]);
      })
      .finally(() => setLoading(false));
  }, []);

  const handlePickSumatra = async () => {
    try {
      const selected = (await open({
        multiple: false,
        filters: [{ name: "Executables", extensions: ["exe"] }],
      })) as string | null;

      if (selected) {
        setSumatraPath(selected);
        localStorage.setItem("sumatra_path", selected);
      }
    } catch (e) {
      console.error("Failed to pick Sumatra exe:", e);
    }
  };

  const handleReceiptChange = (e: SelectChangeEvent<string>) => {
    setReceiptPrinter(e.target.value);
    localStorage.setItem("receipt_printer", e.target.value);
  };

  const handleFullPageChange = (e: SelectChangeEvent<string>) => {
    setFullPagePrinter(e.target.value);
    localStorage.setItem("fullpage_printer", e.target.value);
  };

  if (loading) {
    return (
      <Backdrop
        open
        sx={{ color: "#fff", zIndex: (theme) => theme.zIndex.drawer + 1 }}
      >
        <CircularProgress color="inherit" />
      </Backdrop>
    );
  }

  return (
    <Box sx={{ p: 4, maxWidth: 600, mx: "auto" }}>
      <Typography variant="h4" gutterBottom>
        Printer Config
      </Typography>

      <FormControl
        fullWidth
        margin="normal"
        disabled={loading}
        sx={{ mt: 6, mb: 3 }}
      >
        <InputLabel id="receipt-printer-label">Receipt Printer</InputLabel>
        <Select
          labelId="receipt-printer-label"
          value={receiptPrinter}
          label="Receipt Printer"
          onChange={handleReceiptChange}
        >
          {printers.map((p) => (
            <MenuItem key={p.name} value={p.name}>
              {p.name}
            </MenuItem>
          ))}
        </Select>
        <FormHelperText>
          Select the printer used for customer receipts
        </FormHelperText>
      </FormControl>

      <FormControl fullWidth margin="normal" disabled={loading}>
        <InputLabel id="fullpage-printer-label">Full Page Printer</InputLabel>
        <Select
          labelId="fullpage-printer-label"
          value={fullPagePrinter}
          label="Full Page Printer"
          onChange={handleFullPageChange}
        >
          {printers.map((p) => (
            <MenuItem key={p.name} value={p.name}>
              {p.name}
            </MenuItem>
          ))}
        </Select>
        <FormHelperText>
          If no printer attached select one of the “Print to PDF” options so you
          can save your reports.
        </FormHelperText>
      </FormControl>
      <Box sx={{ mt: 4, mb: 1, display: "flex", alignItems: "center", gap: 2 }}>
        <AppButton
          sx={{ width: 170, fontSize: "0.75rem" }}
          variant="contained"
          onClick={handlePickSumatra}
          disabled={loading}
          aria-label="Select Sumatra Application Location"
          text="Sumatra Location"
        />
        <Typography
          variant="body2"
          sx={{ wordBreak: "break-all", fontSize: "0.75rem" }}
        >
          {sumatraPath ? sumatraPath : "No file selected"}
        </Typography>
      </Box>
      <FormHelperText>
        Sumatra is required to print reports. If you do not have it you can
        download it from: https://www.sumatrapdfreader.org/free-pdf-reader.html
      </FormHelperText>
    </Box>
  );
}
