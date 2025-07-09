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
    <Box className="flex gap-4 mb-3">
      <TextField
        className="flex-1 basis-0"
        label="Search"
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
      />
      <FormControl className="flex-1 basis-0">
        <InputLabel>Category</InputLabel>
        <Select
          value={category}
          label="Category"
          onChange={(e) => onCategoryChange(e.target.value)}
          style={{ minWidth: "13rem" }}
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
