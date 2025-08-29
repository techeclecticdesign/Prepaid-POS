import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { formatCurrency } from "../../lib/util";
import Box from "@mui/material/Box";

type StatsDto = {
  account_total: number;
  total_customer_balances: number;
};

export default function AdminPage() {
  const [stats, setStats] = useState<StatsDto | null>(null);

  useEffect(() => {
    invoke<StatsDto>("get_stats")
      .then(setStats)
      .catch((err) => console.error("Failed to load stats", err));
  }, []);

  return (
    <Box
      sx={{
        minHeight: "100vh",
        display: "flex",
        justifyContent: "center",
        p: 4,
      }}
    >
      <Box sx={{ textAlign: "center" }}>
        {stats && (
          <Box sx={{ mt: 3, fontSize: "1.5rem", fontWeight: "bold" }}>
            <div>Account Total: {formatCurrency(stats.account_total)}</div>
            <div>
              Total Customer Balances:{" "}
              {formatCurrency(stats.total_customer_balances)}
            </div>
          </Box>
        )}
      </Box>
    </Box>
  );
}
