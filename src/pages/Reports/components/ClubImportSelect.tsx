import { useEffect, useState } from "react";
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  MenuItem,
  Select,
  Button,
  CircularProgress,
  Typography,
} from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import ClubImport from "../../../models/ClubImport";

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function ClubImportSelectModal({ open, onClose }: Props) {
  const [imports, setImports] = useState<ClubImport[]>([]);
  const [selectedId, setSelectedId] = useState<number | "">("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!open) return;
    setLoading(true);
    invoke<ClubImport[]>("list_club_imports")
      .then((res) => setImports(res))
      .catch((e) => console.error("Failed to load club imports:", e))
      .finally(() => setLoading(false));
  }, [open]);

  const toDateOnly = (v: unknown) =>
    typeof v === "string" ? v.split("T")[0] : String(v ?? "");

  const handlePrint = async () => {
    if (selectedId === "") return;

    try {
      // find the selected import record
      const imp = imports.find((imp) => imp.id === selectedId);
      if (!imp) {
        console.error("Selected import not found");
        return;
      }
      // activity_from is a JSON date shape; convert it to a JS Date then to ISO
      const fromIso = new Date(imp.activity_from).toISOString();

      await invoke("print_club_import", {
        importId: selectedId,
        startDate: fromIso,
        printerName: localStorage.getItem("fullpage_printer") ?? "",
        sumatraLocation: localStorage.getItem("sumatra_path") ?? "",
      });
      onClose();
    } catch (e) {
      console.error("Print failed:", e);
    }
  };

  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>Select Club Import</DialogTitle>
      <DialogContent>
        {loading ? (
          <CircularProgress />
        ) : (
          <Select
            fullWidth
            value={selectedId}
            onChange={(e) => setSelectedId(e.target.value as number)}
            displayEmpty
            sx={{ mt: 2 }}
          >
            <MenuItem value="">
              <em>-- Select an import --</em>
            </MenuItem>
            {imports.map((imp) => (
              <MenuItem key={imp.id} value={imp.id}>
                {toDateOnly(imp.activity_from)} - {toDateOnly(imp.activity_to)}
              </MenuItem>
            ))}
          </Select>
        )}
        {imports.length === 0 && !loading && (
          <Typography variant="body2" sx={{ mt: 2 }}>
            No imports found.
          </Typography>
        )}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button
          onClick={handlePrint}
          disabled={selectedId === ""}
          variant="contained"
        >
          Print
        </Button>
      </DialogActions>
    </Dialog>
  );
}
