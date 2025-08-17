import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import OperatorSelector from "./components/OperatorSelector";
import OperatorDetails from "./components/OperatorDetails";
import AddOperatorDialog from "./components/AddOperatorDialog";
import {
  operatorSchema,
  type OperatorFormValues,
} from "../../schema/operatorSchema";
import type Operator from "../../models/Operator";
import useOperators from "../../hooks/useOperators";
import useOperatorActions from "./hooks/useOperatorActions";

export default function OperatorsPage() {
  const { operators, refresh } = useOperators();
  const { create, update } = useOperatorActions();
  const [selected, setSelected] = useState<Operator | null>(null);
  const [showDialog, setShowDialog] = useState(false);

  const handleAdd = async (vals: OperatorFormValues) => {
    const parsed = operatorSchema.parse(vals);
    const now = new Date().toISOString();
    await create({
      mdoc: parsed.mdoc,
      name: parsed.name,
      start: now,
      stop: null,
    });
    setShowDialog(false);
    refresh();
  };

  const handleTerminate = async () => {
    if (!selected) return;
    const now = new Date().toISOString();
    await update({
      mdoc: selected.mdoc,
      name: selected.name,
      start: selected.start,
      stop: now,
    });
    setSelected({ ...selected, stop: now });
  };

  const handleRehire = async () => {
    if (!selected) return;
    const now = new Date().toISOString();
    await update({
      mdoc: selected.mdoc,
      name: selected.name,
      start: now,
      stop: null,
    });
    setSelected({ ...selected, start: now, stop: null });
  };

  return (
    <Box
      sx={{
        p: 12,
        display: "flex",
        justifyContent: "center",
        width: "100%",
        mb: "auto",
      }}
    >
      <Box sx={{ width: 440, mx: "auto" }}>
        <Typography
          variant="h4"
          component="h1"
          sx={{
            fontWeight: "bold",
            textAlign: "center",
            mb: 6,
            color: "text.primary",
          }}
        >
          Operators
        </Typography>
        <OperatorSelector
          options={operators}
          selected={selected}
          onChange={setSelected}
          onAddClick={() => setShowDialog(true)}
        />
        {selected && (
          <OperatorDetails
            operator={selected}
            onTerminate={handleTerminate}
            onRehire={handleRehire}
          />
        )}
        <AddOperatorDialog
          open={showDialog}
          onClose={() => setShowDialog(false)}
          onSubmit={handleAdd}
          existingMdocs={operators.map((o) => o.mdoc)}
        />
      </Box>
    </Box>
  );
}
