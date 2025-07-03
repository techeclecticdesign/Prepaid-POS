import { useState } from "react";
import OperatorSelector from "./components/OperatorSelector";
import OperatorDetails from "./components/OperatorDetails";
import AddOperatorDialog from "./components/AddOperatorDialog";
import {
  operatorSchema,
  OperatorFormValues,
} from "../../schema/operatorSchema";
import { useOperators, OperatorDto } from "../../hooks/useOperators";
import { useOperatorActions } from "./hooks/useOperatorActions";

export default function OperatorsPage() {
  const { operators, refresh } = useOperators();
  const { create, update } = useOperatorActions();
  const [selected, setSelected] = useState<OperatorDto | null>(null);
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
    <main className="p-12 flex justify-center bg-gray-50 h-screen">
      <div className="w-1/2">
        <h1 className="text-3xl font-bold mb-8 text-center">Operators</h1>

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
        />
      </div>
    </main>
  );
}
