import { Link, useNavigate } from "react-router-dom";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import { useAuth } from "../AuthProvider";
import ThemeSwitch from "./ThemeSwitch";
import { useTheme as useMuiTheme } from "@mui/material/styles";

export default function AdminSidebar() {
  const { logout } = useAuth();
  const navigate = useNavigate();
  const theme = useMuiTheme();

  return (
    <Box
      component="aside"
      className="fixed top-0 right-0 w-xs shrink-0 p-6 h-screen"
      sx={{
        borderLeft: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
        color: "text.primary",
      }}
    >
      <div className="flex items-center justify-between mb-4">
        <Typography
          variant="h5"
          component="h2"
          className="font-semibold mb-4"
          sx={{ color: "text.primary" }}
        >
          Admin Pages
        </Typography>
        <ThemeSwitch />
      </div>
      <nav className="flex flex-col space-y-2">
        <Link
          to="/operators"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Operators
        </Link>
        <Link
          to="/categories"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Categories
        </Link>
        <Link
          to="/import"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Import
        </Link>
        <Typography
          component="span"
          className="cursor-not-allowed"
          sx={{ color: "text.secondary" }}
        >
          Dolor
        </Typography>
      </nav>
      <Button
        onClick={() => {
          logout();
          navigate("/");
        }}
        className="mt-6 text-left"
        sx={{
          color: "error.main",
          "&:hover": {
            textDecoration: "underline",
            backgroundColor: "transparent",
          },
        }}
        variant="text"
      >
        Sign Out
      </Button>
    </Box>
  );
}
