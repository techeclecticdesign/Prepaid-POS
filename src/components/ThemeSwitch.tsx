import IconButton from "@mui/material/IconButton";
import Brightness4Icon from "@mui/icons-material/Brightness4";
import Brightness7Icon from "@mui/icons-material/Brightness7";
import { useTheme } from "../theme/ThemeContext";

export default function ThemeSwitch() {
  const { mode, toggleMode } = useTheme();

  return (
    <IconButton sx={{ ml: 1 }} onClick={toggleMode} color="inherit">
      {mode === "dark" ? <Brightness7Icon /> : <Brightness4Icon />}
    </IconButton>
  );
}
