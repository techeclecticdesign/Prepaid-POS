import { Link as RouterLink, useNavigate, useLocation } from "react-router-dom";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Link from "@mui/material/Link";
import { useAuth } from "../AuthProvider";
import ThemeSwitch from "./ThemeSwitch";

export default function OperatorSidebar() {
  const navigate = useNavigate();
  const { setActiveOperator } = useAuth();
  const location = useLocation();

  const linkSx = (path: string) => {
    const isCurrent = location.pathname === path;
    return {
      fontWeight: 600,
      py: 0.5,
      fontSize: "1.2rem",
      cursor: isCurrent ? "default" : "pointer",
      textDecoration: isCurrent ? "none" : undefined,
      "&:hover": {
        textDecoration: isCurrent ? "none" : "underline",
      },
    };
  };

  return (
    <Box
      component="aside"
      sx={{
        position: "fixed",
        top: 0,
        right: 0,
        width: "18rem",
        flexShrink: 0,
        p: 3,
        height: "100vh",
        borderLeft: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
        color: "text.primary",
      }}
    >
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          mb: 4,
        }}
      >
        <Typography
          variant="h5"
          component="h2"
          sx={{ color: "text.primary", fontWeight: 600 }}
        >
          Pages
        </Typography>
        <ThemeSwitch />
      </Box>
      <Box component="nav" sx={{ display: "flex", flexDirection: "column" }}>
        <Link
          component={RouterLink}
          to="/sales"
          underline="hover"
          color={location.pathname === "/sales" ? "text.secondary" : "primary"}
          sx={linkSx("/sales")}
        >
          Sales
        </Link>
        <Link
          component={RouterLink}
          to="/products"
          underline="hover"
          color={
            location.pathname === "/products" ? "text.secondary" : "primary"
          }
          sx={linkSx("/products")}
        >
          Products
        </Link>
        <Link
          component={RouterLink}
          to="/customers"
          underline="hover"
          color={
            location.pathname === "/customers" ? "text.secondary" : "primary"
          }
          sx={linkSx("/customers")}
        >
          Customers
        </Link>
        <Link
          component={RouterLink}
          to="/accounts"
          underline="hover"
          color={
            location.pathname === "/accounts" ? "text.secondary" : "primary"
          }
          sx={linkSx("/accounts")}
        >
          Accounts
        </Link>
        <Link
          component={RouterLink}
          to="/reports"
          underline="hover"
          color={
            location.pathname === "/reports" ? "text.secondary" : "primary"
          }
          sx={linkSx("/reports")}
        >
          Reports
        </Link>
        <Link
          underline="hover"
          color="error"
          sx={{ fontSize: "1.2rem", fontWeight: 600, cursor: "pointer" }}
          onClick={() => {
            setActiveOperator(null);
            navigate("/");
          }}
        >
          Sign Out
        </Link>
      </Box>
    </Box>
  );
}
