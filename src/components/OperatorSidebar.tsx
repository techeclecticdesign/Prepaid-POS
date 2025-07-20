import { Link, useNavigate } from "react-router-dom";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import { useTheme as useMuiTheme } from "@mui/material/styles";
import { useAuth } from "../AuthProvider";
import ThemeSwitch from "./ThemeSwitch";

export default function OperatorSidebar() {
  const navigate = useNavigate();
  const { setActiveOperator } = useAuth();
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
          Pages
        </Typography>
        <ThemeSwitch />
      </div>
      <nav className="flex flex-col space-y-2">
        <Link
          to="/sales"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Sales
        </Link>
        <Link
          to="/products"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Products
        </Link>
        <Link
          to="/customers"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Customers
        </Link>
        <Link
          to="/accounts"
          className="hover:underline"
          style={{ color: theme.palette.primary.main }}
        >
          Accounts
        </Link>
      </nav>
      <Button
        onClick={() => {
          navigate("/");
          setActiveOperator(null);
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
