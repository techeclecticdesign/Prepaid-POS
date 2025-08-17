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
    <Box sx={{ "& > * + *": { mt: 6 } }}>
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: { xs: "1fr", md: "1fr 1fr" },
          gap: 6,
          maxWidth: "40rem",
        }}
      >
        {["MDOC", "Name", "Start Time", "Stop Time"].map((label, i) => {
          const value =
            i === 0
              ? operator.mdoc
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
