import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import DialogContent from "@mui/material/DialogContent";
import DialogActions from "@mui/material/DialogActions";
import Button from "@mui/material/Button";
import { LocalizationProvider, DatePicker } from "@mui/x-date-pickers";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFns";

interface Props {
  open: boolean;
  dateReport: string;
  onClose: () => void;
  // passing snack setters from parent to keep single source of truth for snackbar
  setSnackOpen: (v: boolean) => void;
  setSnackMsg: (m: string) => void;
}

export default function DateRangeDialog({
  open,
  dateReport,
  onClose,
  setSnackOpen,
  setSnackMsg,
}: Props) {
  const [startDate, setStartDate] = useState<Date | null>(null);
  const [endDate, setEndDate] = useState<Date | null>(null);

  const runReport = async () => {
    if (!startDate || !endDate) {
      setSnackMsg("Please select both start and end dates.");
      setSnackOpen(true);
      return;
    }
    try {
      const invokeCommon = (cmd: string) =>
        invoke(cmd, {
          startDate: startDate.toISOString(),
          endDate: endDate.toISOString(),
          printerName: localStorage.getItem("fullpage_printer") ?? "",
          sumatraLocation: localStorage.getItem("sumatra_path") ?? "",
        });

      if (dateReport === "bycustomer") {
        await invokeCommon("print_sales_detail_report");
      } else if (dateReport === "byproduct") {
        await invokeCommon("print_product_sales_by_category");
      } else if (dateReport === "byday") {
        await invokeCommon("print_daily_sales_report");
      }
      onClose();
    } catch (e) {
      setSnackMsg(`Failed: ${e}`);
      setSnackOpen(true);
    }
  };

  return (
    <Dialog
      open={open}
      onClose={onClose}
      fullWidth
      maxWidth="xs"
      slotProps={{
        paper: {
          sx: {
            transform: "translateY(-8vh)", // nudge up so calendar isn’t clipped
            maxHeight: "85vh", // avoid overflowing on small screens
          },
        },
      }}
    >
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
        <Button onClick={onClose}>Cancel</Button>
        <Button onClick={runReport}>Run</Button>
      </DialogActions>
    </Dialog>
  );
}
