import Autocomplete from "@mui/material/Autocomplete";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import AppButton from "../../../components/AppButton";
import type Operator from "../../../models/Operator";

interface Props {
  options: Operator[];
  selected: Operator | null;
  onChange: (op: Operator | null) => void;
  onAddClick: () => void;
}

export default function OperatorSelector({
  options,
  selected,
  onChange,
  onAddClick,
}: Props) {
  return (
    <Box sx={{ display: "flex", gap: 2, mb: 6 }}>
      <Autocomplete
        sx={{ width: "100%", maxWidth: "24rem" }}
        options={options}
        getOptionLabel={(o) => o.name}
        value={selected}
        onChange={(_, o) => onChange(o)}
        renderInput={(params) => (
          <TextField {...params} label="Select operator" variant="outlined" />
        )}
      />
      <AppButton onClick={onAddClick} text="Add Operator" />
    </Box>
  );
}
