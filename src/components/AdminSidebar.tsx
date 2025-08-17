import { Link as RouterLink, useNavigate, useLocation } from "react-router-dom";
import Link from "@mui/material/Link";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import { useAuth } from "../AuthProvider";
import ThemeSwitch from "./ThemeSwitch";

export default function AdminSidebar() {
  const { logout } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();
  const isActive = (path: string) => location.pathname.startsWith(path);

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
          Admin Pages
        </Typography>
        <ThemeSwitch />
      </Box>
      <Box component="nav" sx={{ display: "flex", flexDirection: "column" }}>
        <Link
          component={RouterLink}
          to="/operators"
          underline="hover"
          color={isActive("/operators") ? "text.secondary" : "primary"}
          sx={linkSx("/operators")}
        >
          Operators
        </Link>
        <Link
          component={RouterLink}
          to="/categories"
          underline="hover"
          color={isActive("/categories") ? "text.secondary" : "primary"}
          sx={linkSx("/categories")}
        >
          Categories
        </Link>
        <Link
          component={RouterLink}
          to="/import"
          underline="hover"
          color={isActive("/import") ? "text.secondary" : "primary"}
          sx={linkSx("/import")}
        >
          Import
        </Link>
        <Link
          component={RouterLink}
          to="/lost-inventory"
          underline="hover"
          color={isActive("/lost-inventory") ? "text.secondary" : "primary"}
          sx={linkSx("/lost-inventory")}
        >
          Lost Inventory
        </Link>
        <Link
          component={RouterLink}
          to="/price-adjustments"
          underline="hover"
          color={isActive("/price-adjustments") ? "text.secondary" : "primary"}
          sx={linkSx("/price-adjustments")}
        >
          Price Adjustments
        </Link>
        <Link
          component={RouterLink}
          to="/barcode"
          underline="hover"
          color={isActive("/barcode") ? "text.secondary" : "primary"}
          sx={linkSx("/barcode")}
        >
          Barcode Scanner Config
        </Link>
        <Link
          component={RouterLink}
          to="/printer"
          underline="hover"
          color={isActive("/printer") ? "text.secondary" : "primary"}
          sx={linkSx("/printer")}
        >
          Printer Config
        </Link>
        <Link
          component={RouterLink}
          to="/change-password"
          underline="hover"
          color={isActive("/change-password") ? "text.secondary" : "primary"}
          sx={linkSx("/change-password")}
        >
          Change Password
        </Link>
        <Link
          component={RouterLink}
          to="/weekly-limit"
          underline="hover"
          color={isActive("/weekly-limit") ? "text.secondary" : "primary"}
          sx={linkSx("/weekly-limit")}
        >
          Weekly Limit
        </Link>
        <Link
          onClick={() => {
            logout();
            navigate("/");
          }}
          underline="hover"
          color="error"
          sx={{ fontSize: "1.2rem", fontWeight: 600, cursor: "pointer" }}
        >
          Sign Out
        </Link>
      </Box>
    </Box>
  );
}
