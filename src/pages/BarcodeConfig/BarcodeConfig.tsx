import { useState, useEffect } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Radio from "@mui/material/Radio";
import RadioGroup from "@mui/material/RadioGroup";
import FormControlLabel from "@mui/material/FormControlLabel";
import FormControl from "@mui/material/FormControl";
import FormLabel from "@mui/material/FormLabel";
import InputLabel from "@mui/material/InputLabel";
import Select, { type SelectChangeEvent } from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import { useTheme } from "@mui/material/styles";

type ScannerType = "Zebra" | "Generic";

export default function BarcodeConfig() {
  const theme = useTheme();
  const [scannerType, setScannerType] = useState<ScannerType | string>("");
  const [zebraMode, setZebraMode] = useState("load");

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
    <Box
      sx={{
        p: { xs: 2, sm: 4 },
        width: { xs: "100%", md: "80%" },
        mx: "auto",
      }}
    >
      <Typography variant="h4" component="h1" sx={headingSx}>
        Barcode Scanner Configuration
      </Typography>

      <Box
        sx={{
          display: "flex",
          justifyContent: "space-between",
          gap: 4,
          mb: 12,
          mt: 6,
        }}
      >
        <FormControl sx={{ minWidth: 300 }}>
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
          <FormControl>
            <FormLabel>Zebra Mode</FormLabel>
            <RadioGroup
              row
              value={zebraMode}
              onChange={(e) => setZebraMode(e.target.value)}
            >
              <FormControlLabel
                value="load"
                control={<Radio />}
                label="Load Settings"
              />
              <FormControlLabel
                value="reset"
                control={<Radio />}
                label="Factory Reset"
              />
            </RadioGroup>
          </FormControl>
        )}
      </Box>

      {scannerType === "Zebra" && (
        <Box sx={{ textAlign: "center" }}>
          <Typography variant="body1" sx={bodyTextSx}>
            {zebraMode === "reset"
              ? "Scan the barcode to restore your scanner to its factory defaults."
              : "If you have not already done so, please scan the QR code below to \
            configure your Zebra scanner. This will ensure optimal settings for \
            use in a Point of Sale environment."}
          </Typography>
          <Box
            component="img"
            src={zebraMode === "reset" ? "/factoryReset.png" : "/barcode.png"}
            alt="scanner barcode"
            sx={{
              maxWidth: 300,
              height: "auto",
              display: "block",
              mx: "auto",
            }}
          />
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
