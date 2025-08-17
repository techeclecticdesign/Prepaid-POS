import Box from "@mui/material/Box";

export default function AdminPage() {
  return (
    <Box
      sx={{
        minHeight: "100vh",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        p: 4,
      }}
    >
      <Box sx={{ textAlign: "center" }}>
        <Box
          component="img"
          src="mock.png"
          alt="mock preview"
          sx={{ maxWidth: 400, width: "100%", height: "auto", borderRadius: 2 }}
        />
      </Box>
    </Box>
  );
}
