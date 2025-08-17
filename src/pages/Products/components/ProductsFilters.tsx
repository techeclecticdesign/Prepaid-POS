import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import type Category from "../../../models/Category";

interface Props {
  search: string;
  category: string;
  categories: Category[];
  onSearchChange: (search: string) => void;
  onCategoryChange: (category: string) => void;
}

export default function ProductsFilters({
  search,
  category,
  categories,
  onSearchChange,
  onCategoryChange,
}: Props) {
  return (
    <Box sx={{ display: "flex", gap: 2, mb: 1.5 }}>
      <TextField
        sx={{ flex: 1, flexBasis: 0 }}
        label="Search"
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
      />
      <FormControl sx={{ flex: 1, flexBasis: 0 }}>
        <InputLabel>Category</InputLabel>
        <Select
          value={category}
          label="Category"
          onChange={(e) => onCategoryChange(e.target.value)}
          sx={{ minWidth: "13rem" }}
        >
          <MenuItem value="">
            <Typography component="em" sx={{ color: "text.secondary" }}>
              All
            </Typography>
          </MenuItem>
          {categories.map((c) => (
            <MenuItem key={c.id} value={c.name}>
              {c.name}
            </MenuItem>
          ))}
        </Select>
      </FormControl>
    </Box>
  );
}
