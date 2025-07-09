import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import AppButton from "../../../components/AppButton";
import type Operator from "../../../models/Operator";

interface Props {
  operator: Operator;
  onTerminate: () => void;
  onRehire: () => void;
}

export default function OperatorDetails({
  operator,
  onTerminate,
  onRehire,
}: Props) {
  return (
    <Box className="space-y-6">
      <Box className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-2xl">
        {["MDOC", "Name", "Start Time", "Stop Time"].map((label, i) => {
          const value =
            i === 0
              ? operator.id
              : i === 1
                ? operator.name
                : i === 2
                  ? new Date(operator.start).toLocaleString()
                  : operator.stop
                    ? new Date(operator.stop).toLocaleString()
                    : "—";
          return (
            <TextField
              key={i}
              label={label}
              value={value}
              slotProps={{ input: { readOnly: true } }}
              variant="filled"
            />
          );
        })}
        <AppButton onClick={onTerminate} text="Terminate Operator" />
        <AppButton onClick={onRehire} text="Rehire Operator" />
      </Box>
    </Box>
  );
}
