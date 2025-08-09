use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::ClubTransaction;
use crate::domain::report_models::club_import_report::{ClubTransactionWithTotal, PeriodTotals};
use crate::domain::repos::ClubTransactionRepoTrait;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteClubTransactionRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteClubTransactionRepo {
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl ClubTransactionRepoTrait for SqliteClubTransactionRepo {
    fn list(&self) -> Result<Vec<ClubTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, import_id, entity_name, mdoc, tx_type, amount, date FROM club_transactions ORDER BY date DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(ClubTransaction {
                id: r.get(0)?,
                import_id: r.get(1)?,
                entity_name: r.get(2)?,
                mdoc: r.get(3)?,
                tx_type: r.get(4)?,
                amount: r.get(5)?,
                date: r.get(6)?,
            })
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_by_import_id_with_total(
        &self,
        import_id: i32,
        start_date: Option<NaiveDateTime>,
    ) -> Result<Vec<ClubTransactionWithTotal>, AppError> {
        let conn = self.conn.safe_lock()?;
        let sql = r#"
        WITH txs_all AS (
          SELECT
            id,
            import_id,
            entity_name,
            mdoc,
            tx_type,
            amount,
            date
          FROM club_transactions
        ),
        windowed AS (
          SELECT
            t.*,
            SUM(t.amount) OVER (ORDER BY t.date ASC, t.id ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS running_total
          FROM txs_all t
        )
        SELECT
          id,
          import_id,
          entity_name,
          mdoc,
          tx_type,
          amount,
          date,
          running_total
        FROM windowed
        WHERE import_id = ?1
          AND (?2 IS NULL OR date >= ?2)
        ORDER BY id ASC
        "#;

        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![import_id, start_date], |r| {
            Ok(ClubTransactionWithTotal {
                id: r.get("id")?,
                import_id: r.get("import_id")?,
                entity_name: r.get("entity_name")?,
                mdoc: r.get("mdoc")?,
                tx_type: r.get("tx_type")?,
                amount: r.get("amount")?,
                date: r.get("date")?,
                running_total: r.get("running_total")?,
            })
        })?;

        let mut out = Vec::with_capacity(rows.size_hint().0);
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;

        if tx.id > 0 {
            // Preserve provided id for migration
            conn.execute(
                "INSERT INTO club_transactions (id, import_id, entity_name, mdoc, tx_type, amount, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    tx.id,
                    tx.import_id,
                    tx.entity_name,
                    tx.mdoc,
                    format!("{:?}", tx.tx_type),
                    tx.amount,
                    tx.date
                ],
            )?;
            log::info!("Inserted club_transaction with explicit id={}", tx.id);
        } else {
            conn.execute(
                "INSERT INTO club_transactions (import_id, entity_name, mdoc, tx_type, amount, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    tx.import_id,
                    tx.entity_name,
                    tx.mdoc,
                    format!("{:?}", tx.tx_type),
                    tx.amount,
                    tx.date
                ],
            )?;
        }
        Ok(())
    }

    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT t.id,
                   t.import_id,
                   t.entity_name,
                   t.mdoc,
                   t.tx_type,
                   t.amount,
                   t.date,
                   c.name     AS customer_name
            FROM club_transactions t
            LEFT JOIN customer c ON t.mdoc = c.mdoc
            WHERE 1=1
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();
        if let Some(ref d) = date {
            sql.push_str(" AND date(t.date)=date(?)");
            params.push(d);
        }
        // date filter
        if let Some(ref s) = search {
            sql.push_str(" AND (c.name LIKE ? OR t.mdoc LIKE ?)");
            let pat = format!("%{s}%");
            string_params.push(pat);
            let last = string_params.last().unwrap();
            params.push(last);
            params.push(last);
        }
        sql.push_str(" ORDER BY t.date DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            let ct = ClubTransaction {
                id: r.get(0)?,
                import_id: r.get(1)?,
                entity_name: r.get(2)?,
                mdoc: r.get(3)?,
                tx_type: r.get(4)?,
                amount: r.get(5)?,
                date: r.get(6)?,
            };
            let name: Option<String> = r.get(7)?;
            Ok((ct, name))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "SELECT COUNT(*) FROM club_transactions t LEFT JOIN customer c ON t.mdoc=c.mdoc WHERE 1=1".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();
        if let Some(ref d) = date {
            sql.push_str(" AND date(t.date)=date(?)");
            params.push(d);
        }
        if let Some(ref s) = search {
            sql.push_str(" AND (c.name LIKE ? OR t.mdoc LIKE ?)");
            let pat = format!("%{s}%");
            string_params.push(pat);
            let last = string_params.last().unwrap();
            params.push(last);
            params.push(last);
        }
        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| {
            let count: i64 = r.get(0)?;
            Ok(count as i32)
        })
        .map_err(Into::into)
    }

    fn get_account_total(&self) -> Result<i32, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare("SELECT SUM(amount) FROM club_transactions")?;
        let total: Option<i64> = stmt.query_row([], |row| row.get(0))?;
        Ok(total.unwrap_or(0) as i32)
    }

    // get customer withdrawals and deposits
    fn get_period_sums_for_import(&self, import_id: i32) -> Result<PeriodTotals, AppError> {
        let conn = self.conn.safe_lock()?;

        let sql = r#"
        SELECT
          COALESCE(SUM(CASE WHEN amount > 0 THEN amount ELSE 0 END), 0) AS period_pos_sum,
          COALESCE(SUM(CASE WHEN amount < 0 THEN amount ELSE 0 END), 0) AS period_neg_sum
        FROM club_transactions
        WHERE import_id = ?1
        "#;

        let mut stmt = conn.prepare(sql)?;
        let totals = stmt.query_row(params![import_id], |r| {
            Ok(PeriodTotals {
                period_pos_sum: r.get(0)?,
                period_neg_sum: r.get(1)?,
            })
        })?;

        Ok(totals)
    }
}
