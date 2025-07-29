use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::Customer;
use crate::domain::repos::CustomerRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteCustomerRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteCustomerRepo {
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl CustomerRepoTrait for SqliteCustomerRepo {
    fn list(&self) -> Result<Vec<Customer>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt =
            conn.prepare("SELECT mdoc, name, added, updated FROM customer ORDER BY name DESC")?;
        let rows = stmt.query_map([], |r| {
            Ok(Customer {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                added: r.get(2)?,
                updated: r.get(3)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Customer>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt =
            conn.prepare("SELECT mdoc, name, added, updated FROM customer WHERE mdoc = ?1")?;
        let mut rows = stmt.query(params![mdoc])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Customer {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                added: r.get(2)?,
                updated: r.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn search(
        &self,
        limit: i32,
        offset: i32,
        search: Option<String>,
    ) -> Result<Vec<(Customer, i32)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        // compute balance = sum(deposits) - sum(spent)
        let mut sql = "\
            SELECT c.mdoc,
                   c.name,
                   c.added,
                   c.updated,
                   (COALESCE(ct.added,0) - COALESCE(sp.spent,0)) AS balance
            FROM customer c
            LEFT JOIN (
              SELECT mdoc,
                     SUM(CASE 
                           WHEN tx_type = 'Deposit'    THEN amount
                           WHEN tx_type = 'Withdrawal' THEN amount
                           ELSE 0
                         END) AS added
              FROM club_transactions
              GROUP BY mdoc
            ) ct ON c.mdoc = ct.mdoc
            LEFT JOIN (
              SELECT t.customer_mdoc AS mdoc,
                     SUM(d.quantity * d.price) AS spent
              FROM customer_transactions t
              JOIN customer_tx_detail d 
                ON t.order_id = d.order_id
              GROUP BY t.customer_mdoc
            ) sp ON c.mdoc = sp.mdoc
            WHERE 1=1
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        // filter for any‐position matches
        if let Some(s) = search.as_ref().filter(|s| !s.trim().is_empty()) {
            let contains = format!("%{}%", s);
            string_params.push(contains.clone());
            string_params.push(contains.clone());
            sql.push_str(" AND (c.mdoc LIKE ? OR c.name LIKE ?)");
        }

        // order prefix‐matches first, then contains, then by name
        if let Some(s) = search.as_ref().filter(|s| !s.trim().is_empty()) {
            let prefix = format!("{}%", s);
            string_params.push(prefix.clone());
            string_params.push(prefix.clone());
            sql.push_str(
                " ORDER BY
               (CASE WHEN c.mdoc LIKE ? OR c.name LIKE ? THEN 0 ELSE 1 END),
               c.name ASC
            LIMIT ? OFFSET ?",
            );
        } else {
            sql.push_str(" ORDER BY c.name ASC LIMIT ? OFFSET ?");
        }
        // bind all collected LIKE‐patterns (contains×2, prefix×2) in insertion order
        for pat in &string_params {
            params.push(pat);
        }
        // then bind pagination
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            let cust = Customer {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                added: r.get(2)?,
                updated: r.get(3)?,
            };
            let balance: i32 = r.get(4)?;
            Ok((cust, balance))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, search: Option<String>) -> Result<i32, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "SELECT COUNT(*) FROM customer WHERE 1=1".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        if let Some(ref s) = search {
            sql.push_str(" AND (mdoc LIKE ? OR name LIKE ?)");
            let pat = format!("%{s}%");
            string_params.push(pat);
            let p = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("customer count pattern missing".into()))?;
            params.push(p);
            params.push(p);
        }

        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| {
            let count: i64 = r.get(0)?;
            Ok(count as i32) // cast as i32
        })
        .map_err(Into::into)
    }

    fn list_customer_accounts(&self) -> Result<Vec<(Customer, i32)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let sql = "\
            SELECT c.mdoc,
                   c.name,
                   c.added,
                   c.updated,
                   COALESCE(ct.added,0) - COALESCE(sp.spent,0) AS balance
            FROM customer c
            LEFT JOIN (
              SELECT mdoc,
                     SUM(
                       CASE 
                         WHEN tx_type = 'Deposit'    THEN amount
                         WHEN tx_type = 'Withdrawal' THEN amount
                         ELSE 0
                       END
                     ) AS added
              FROM club_transactions
              GROUP BY mdoc
            ) ct ON c.mdoc = ct.mdoc
            LEFT JOIN (
              SELECT t.customer_mdoc AS mdoc,
                     SUM(d.quantity * d.price) AS spent
              FROM customer_transactions t
              JOIN customer_tx_detail d ON t.order_id = d.order_id AND d.order_id IS NOT NULL
              GROUP BY t.customer_mdoc
            ) sp ON c.mdoc = sp.mdoc
            WHERE 1=1
        ";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |r| {
            Ok((
                Customer {
                    mdoc: r.get(0)?,
                    name: r.get(1)?,
                    added: r.get(2)?,
                    updated: r.get(3)?,
                },
                {
                    let balance: i64 = r.get(4)?;
                    balance as i32
                },
            ))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn update(&self, customer: &Customer) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "UPDATE customer SET name = ?1, added = ?2, updated = ?3 WHERE mdoc = ?4",
            params![
                customer.name,
                customer.added,
                customer.updated,
                customer.mdoc
            ],
        )?;
        Ok(())
    }

    fn create(&self, customer: &Customer) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO customer (mdoc, name, added, updated) VALUES (?1, ?2, ?3, ?4)",
            params![
                customer.mdoc,
                customer.name,
                customer.added,
                customer.updated
            ],
        )?;
        Ok(())
    }

    // Sum total of all customer account balances (deposits minus spent)
    fn sum_all_balances(&self) -> Result<i32, AppError> {
        let conn = self.conn.safe_lock()?;
        let sql = "\
        SELECT SUM(balance) FROM (\
          SELECT (COALESCE(ct.added,0) - COALESCE(sp.spent,0)) AS balance \
          FROM customer c \
          LEFT JOIN (\
            SELECT mdoc, \
                   SUM(CASE \
                         WHEN tx_type = 'Deposit' THEN amount \
                         WHEN tx_type = 'Withdrawal' THEN amount \
                         ELSE 0 END) AS added \
            FROM club_transactions \
            GROUP BY mdoc\
          ) ct ON c.mdoc = ct.mdoc \
          LEFT JOIN (\
            SELECT t.customer_mdoc AS mdoc, \
                   SUM(d.quantity * d.price) AS spent \
            FROM customer_transactions t \
            JOIN customer_tx_detail d \
              ON t.order_id = d.order_id \
            GROUP BY t.customer_mdoc\
          ) sp ON c.mdoc = sp.mdoc \
        )";
        let total: i64 = conn.query_row(sql, [], |r| r.get(0))?;
        Ok(total as i32)
    }
}
