import { Outlet } from "react-router-dom";
import {
  ThemeProvider as MuiThemeProvider,
  CssBaseline,
  Box,
} from "@mui/material";
import { useTheme } from "../theme/ThemeContext";
import { getMuiTheme } from "../theme/muiTheme";

interface SidebarLayoutProps {
  Sidebar: () => JSX.Element;
  activeOperatorMdoc: number | null;
}

export default function SidebarLayout({ Sidebar }: SidebarLayoutProps) {
  const { mode } = useTheme();
  const muiTheme = getMuiTheme(mode);

  return (
    <MuiThemeProvider theme={muiTheme}>
      <CssBaseline />
      <Box sx={{ display: "flex", height: "100vh" }}>
        <Box
          component="main"
          sx={{
            flex: 1,
            overflow: "auto",
            p: 3,
            mr: "16rem",
          }}
        >
          <Box
            sx={{
              backgroundColor: "background.default",
              color: "text.primary",
              minHeight: "100%",
            }}
          >
            <Outlet />
          </Box>
        </Box>
        {Sidebar()}
      </Box>
    </MuiThemeProvider>
  );
}
