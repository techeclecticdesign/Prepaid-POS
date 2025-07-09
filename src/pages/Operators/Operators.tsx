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
      id: parsed.mdoc,
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
      id: selected.id,
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
      id: selected.id,
      name: selected.name,
      start: now,
      stop: null,
    });
    setSelected({ ...selected, start: now, stop: null });
  };

  return (
    <Box className="p-12 flex justify-center w-full mb-auto">
      <Box className="w-110 mx-auto">
        <Typography
          variant="h4"
          component="h1"
          className="font-bold mb-8 text-center"
          sx={{ color: "text.primary", marginBottom: 6 }}
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
          existingMdocs={operators.map((o) => o.id)}
        />
      </Box>
    </Box>
  );
}
