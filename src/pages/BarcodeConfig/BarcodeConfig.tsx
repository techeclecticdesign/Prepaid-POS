import { useState, useEffect } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select, { type SelectChangeEvent } from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import { useTheme } from "@mui/material/styles";

type ScannerType = "Zebra" | "Generic";

export default function BarcodeConfig() {
  const theme = useTheme();
  const [scannerType, setScannerType] = useState<ScannerType | string>("");

  useEffect(() => {
    // Load from local storage on component mount
    const savedScannerType = localStorage.getItem("barcode");
    if (savedScannerType === "Zebra" || savedScannerType === "Generic") {
      setScannerType(savedScannerType);
    } else {
      // Default to Zebra if nothing is saved or invalid value
      setScannerType("Zebra");
      localStorage.setItem("barcode", "Zebra");
    }
  }, []);

  const handleScannerTypeChange = (event: SelectChangeEvent) => {
    const newType = event.target.value as ScannerType;
    setScannerType(newType);
    localStorage.setItem("barcode", newType);
  };

  const headingSx = {
    fontSize: {
      xs: theme.typography.pxToRem(24),
      xl: theme.typography.pxToRem(36),
    },
    fontWeight: theme.typography.fontWeightBold as number,
    color: "text.primary",
    mb: 4,
  };

  const bodyTextSx = {
    fontSize: {
      xs: theme.typography.pxToRem(14),
      xl: theme.typography.pxToRem(16),
    },
    color: "text.secondary",
    lineHeight: 1.6,
    mt: 3,
    mb: 4,
  };

  return (
    <Box className="p-2 w-4/5 mx-auto 2xl:px-50">
      <Typography variant="h4" component="h1" sx={headingSx}>
        Barcode Scanner Configuration
      </Typography>

      <FormControl fullWidth sx={{ maxWidth: 300, mb: 4 }}>
        <InputLabel id="scanner-type-label">Scanner Type</InputLabel>
        <Select
          labelId="scanner-type-label"
          id="scanner-type-select"
          value={scannerType}
          label="Scanner Type"
          onChange={handleScannerTypeChange}
        >
          <MenuItem value="Zebra">Zebra (recommended)</MenuItem>
          <MenuItem value="Generic">Generic</MenuItem>
        </Select>
      </FormControl>

      {scannerType === "Zebra" && (
        <Box sx={{ textAlign: "center" }}>
          <Typography variant="body1" sx={bodyTextSx}>
            If you have not already done so, please scan the QR code below to
            configure your Zebra scanner. This will ensure optimal settings for
            use in a Point of Sale environment.
          </Typography>
          <Box sx={{ mt: 4 }}>
            <img
              src="barcode.png"
              alt="QR code for Zebra scanner configuration"
              style={{
                maxWidth: "300px",
                height: "auto",
                display: "block",
                margin: "0 auto",
              }}
            />
          </Box>
        </Box>
      )}

      {scannerType === "Generic" && (
        <Box>
          <Typography variant="body1" sx={bodyTextSx}>
            Using a non-Zebra brand scanner will work but use caution as it will
            potentially scan other types of barcode such as QR codes, which can
            create a false impression an item was scanned. Additionally using a
            generic scanner may sometimes fail to capture the whole barcode, but
            this should be rare.
          </Typography>
        </Box>
      )}
    </Box>
  );
}
