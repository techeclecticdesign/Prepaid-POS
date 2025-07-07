import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemText from "@mui/material/ListItemText";
import IconButton from "@mui/material/IconButton";
import DeleteIcon from "@mui/icons-material/Delete";
import type { Category } from "../../../models/Category";

interface Props {
  categories: Category[];
  onDelete: (id: number) => void;
}

export default function CategoryList({ categories, onDelete }: Props) {
  return (
    <List>
      {categories.map((cat) => (
        <ListItem
          key={cat.id}
          secondaryAction={
            <IconButton edge="end" onClick={() => onDelete(cat.id)}>
              <DeleteIcon />
            </IconButton>
          }
        >
          <ListItemText primary={cat.name} />
        </ListItem>
      ))}
    </List>
  );
}
