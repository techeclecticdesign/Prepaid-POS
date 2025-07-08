import {
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from "@mui/material";
import type { Category } from "../../../models/Category";

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
    <div className="flex gap-4 mb-3">
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
          style={{ minWidth: 200 }}
        >
          <MenuItem value="">
            <em>All</em>
          </MenuItem>
          {categories.map((c) => (
            <MenuItem key={c.id} value={c.name}>
              {c.name}
            </MenuItem>
          ))}
        </Select>
      </FormControl>
    </div>
  );
}
