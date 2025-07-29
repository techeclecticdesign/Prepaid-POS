use crate::common::error::AppError;
use crate::domain::models::club_transaction::TransactionType;
use crate::domain::models::{
    ClubImport, ClubTransaction, Customer, CustomerTransaction, CustomerTxDetail,
    InventoryTransaction, Operator, Product,
};
use crate::domain::repos::{
    CategoryRepoTrait, ClubImportRepoTrait, ClubTransactionRepoTrait, CustomerRepoTrait,
    CustomerTransactionRepoTrait, CustomerTxDetailRepoTrait, InventoryTransactionRepoTrait,
    OperatorRepoTrait, ProductRepoTrait,
};
use log::warn;
use odbc_api::{Cursor, Environment, Nullable};
use odbc_sys::Timestamp;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

// SELECT lngOperatorMdoc, txtOperatorName, datStart, datStop FROM tblOperator
type RawOperatorRow = (
    Option<i32>,       // mdoc
    Option<String>,    // name
    Option<Timestamp>, // start
    Option<Timestamp>, // stop
);

// SELECT txtProductUPC, txtProductCategory, txtProductDescription,
//        curProductPrice, datUpdated, datAdded, datDeleted
type RawProductRow = (
    Option<String>,    // upc
    Option<String>,    // category
    Option<String>,    // desc
    Option<f64>,       // price
    Option<Timestamp>, // updated
    Option<Timestamp>, // added
    Option<Timestamp>, // deleted
);

// SELECT DISTINCT txtProductCategory FROM tblProduct
type RawCategoryRow<S> = Option<S>;

// SELECT lngCustomerMDOC, txtCustomerName, datAdded FROM tblCustomer
type RawCustomerRow = (
    Option<i32>,       // mdoc
    Option<String>,    // name
    Option<Timestamp>, // added
);

// SELECT idnClubStatement, txtAccountActivity, txtFileName, datImported FROM tblClubStatement
type RawClubImportRow = (
    Option<i32>,       // id
    Option<String>,    // activity
    Option<String>,    // source_file
    Option<Timestamp>, // imported
);

// SELECT idnRecord, nClubStatement, txtReceivedFrom, txtTransaction, curAmount, datPosted
type RawClubTransactionRow = (
    Option<i32>,       // id
    Option<i32>,       // import_id
    Option<String>,    // received_from
    Option<String>,    // tx_type
    Option<f64>,       // amount
    Option<Timestamp>, // posted
);

// SELECT lngOrderId, lngCustomerMdoc, lngOperatorMdoc, datEntry, txtOrderNote FROM tblCustomerOrder
type RawCustomerOrderRow = (
    Option<i32>,       // legacy_id
    Option<i32>,       // customer_mdoc
    Option<i32>,       // operator_mdoc
    Option<Timestamp>, // entry
    Option<String>,    // note
);

// SELECT lngOrderId, txtRefNum, txtNote, txtProductUPC,
//        lngInventoryAdjustment, datPosted, lngCustomerMdoc, lngOperatorMdoc
type RawInventoryRow = (
    Option<i32>,       // ref_order_id
    Option<String>,    // refnum
    Option<String>,    // note
    Option<String>,    // upc
    Option<i32>,       // adjustment
    Option<Timestamp>, // posted
    Option<i32>,       // customer_mdoc
    Option<i32>,       // operator_mdoc
);

// SELECT idnOrderDetailId, lngOrderId, txtProductUPC, lngQty, curProductPrice FROM tblCustomerOrderDetail
pub type RawCustomerOrderDetailRow = (
    Option<i32>,    // legacy idnOrderDetailId (ignored)
    Option<i32>,    // legacy lngOrderId
    Option<String>, // legacy txtProductUPC
    Option<i32>,    // legacy lngQty
    Option<f64>,    // legacy curProductPrice
);
pub struct LegacyMigrationDeps {
    pub op_repo: Arc<dyn OperatorRepoTrait>,
    pub product_repo: Arc<dyn ProductRepoTrait>,
    pub category_repo: Arc<dyn CategoryRepoTrait>,
    pub customer_repo: Arc<dyn CustomerRepoTrait>,
    pub club_transaction_repo: Arc<dyn ClubTransactionRepoTrait>,
    pub club_imports_repo: Arc<dyn ClubImportRepoTrait>,
    pub inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
    pub customer_transaction_repo: Arc<dyn CustomerTransactionRepoTrait>,
    pub cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
    pub sqlite_conn: Arc<Mutex<Connection>>,
}

pub struct LegacyMigrationUseCases {
    pub deps: LegacyMigrationDeps,
}

/* Note: to keep migrations testable they use two functions, one fetches the legacy data into rows,
 * the other does the parsing of these rows, maps to domain models, and persists to repo. */
impl LegacyMigrationUseCases {
    #[must_use]
    pub const fn new(deps: LegacyMigrationDeps) -> Self {
        Self { deps }
    }

    pub fn has_legacy_data(&self) -> Result<bool, AppError> {
        let path = Path::new(r"C:\Annex\CanteenAnnex.accdb");
        Ok(path.exists())
    }

    pub fn do_legacy_data_import(&self) -> Result<bool, AppError> {
        // connect to the Annex file
        use odbc_api::ConnectionOptions;
        let env = Environment::new().map_err(|e| AppError::Unexpected(e.to_string()))?;
        let conn_str = r"Driver={Microsoft Access Driver (*.mdb, *.accdb)};DBQ=C:\\Annex\\CanteenAnnex.accdb;Uid=Admin;Pwd=;";
        let conn = env
            .connect_with_connection_string(conn_str, ConnectionOptions::default())
            .map_err(|e| AppError::Unexpected(e.to_string()))?;

        {
            let sqlite_conn = self
                .deps
                .sqlite_conn
                .lock()
                .map_err(|e| AppError::Unexpected(format!("mutex poisoned: {e}")))?;
            sqlite_conn
                .execute_batch("BEGIN;")
                .map_err(|e| AppError::Unexpected(e.to_string()))?;
            // Insert operator mdoc 0 so FK constraint is not violated with legacy data
            sqlite_conn
                .execute(
                    "INSERT OR IGNORE INTO operators (mdoc, name, start, stop)
                    VALUES (0, 'unknown operator', '1900-01-01 00:00:00', '1900-01-01 00:00:00');",
                    (),
                )
                .map_err(|e| AppError::Unexpected(e.to_string()))?;
        }
        self.migrate_operators(&conn)?;
        self.migrate_products(&conn)?;
        self.migrate_categories(&conn)?;
        self.migrate_customers(&conn)?;
        self.migrate_club_imports(&conn)?;
        self.migrate_club_transactions(&conn)?;
        self.migrate_inventory_transactions(&conn)?;
        self.migrate_customer_orders(&conn)?;
        self.migrate_customer_order_details(&conn)?;
        {
            let sqlite_conn = self
                .deps
                .sqlite_conn
                .lock()
                .map_err(|e| AppError::Unexpected(format!("mutex poisoned: {e}")))?;
            sqlite_conn
                .execute_batch("COMMIT;")
                .map_err(|e| AppError::Unexpected(e.to_string()))?;
        }
        Ok(true)
    }

    // helper to convert ODBC Timestamp -> NaiveDateTime
    pub(crate) fn ts_to_naive(ts: Timestamp) -> Result<chrono::NaiveDateTime, AppError> {
        let date = chrono::NaiveDate::from_ymd_opt(
            i32::from(ts.year),
            u32::from(ts.month),
            u32::from(ts.day),
        )
        .ok_or_else(|| AppError::Unexpected("invalid date".into()))?;
        let time = chrono::NaiveTime::from_hms_micro_opt(
            u32::from(ts.hour),
            u32::from(ts.minute),
            u32::from(ts.second),
            ts.fraction,
        )
        .ok_or_else(|| AppError::Unexpected("invalid time".into()))?;
        Ok(chrono::NaiveDateTime::new(date, time))
    }

    // Migrate operators from tblOperator
    pub fn migrate_operators(&self, conn: &odbc_api::Connection<'_>) -> Result<(), AppError> {
        // collect raw row data
        let mut raws: Vec<RawOperatorRow> = Vec::new();

        let mut cursor = conn
            .execute(
                "SELECT lngOperatorMdoc, txtOperatorName, datStart, datStop FROM tblOperator",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblOperator".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            // mdoc
            let mut mdoc_buf = Nullable::<i32>::null();
            let mdoc_opt = match row.get_data(1, &mut mdoc_buf) {
                Ok(()) => mdoc_buf.into_opt(),
                Err(e) => {
                    warn!("skip operator row: bad id: {e}");
                    raws.push((None, None, None, None));
                    continue;
                }
            };
            // name
            let mut name_buf = Vec::<u8>::new();
            let name_opt = match row.get_text(2, &mut name_buf) {
                Ok(_) => Some(String::from_utf8_lossy(&name_buf).into_owned()),
                Err(e) => {
                    warn!("skip operator row: bad name: {e}");
                    raws.push((None, None, None, None));
                    continue;
                }
            };
            // start
            let mut start_buf = Nullable::<Timestamp>::null();
            let start_opt = match row.get_data(3, &mut start_buf) {
                Ok(()) => start_buf.into_opt(),
                Err(e) => {
                    warn!("skip operator row: bad start date: {e}");
                    raws.push((None, None, None, None));
                    continue;
                }
            };
            // stop
            let mut stop_buf = Nullable::<Timestamp>::null();
            let stop_opt = match row.get_data(4, &mut stop_buf) {
                Ok(()) => stop_buf.into_opt(),
                Err(e) => {
                    warn!("skip operator row: bad stop date: {e}");
                    raws.push((None, None, None, None));
                    continue;
                }
            };

            raws.push((mdoc_opt, name_opt, start_opt, stop_opt));
        }

        // delegate
        self.migrate_operators_from_rows(raws)
    }

    // Testable logic for operators migrations
    pub(crate) fn migrate_operators_from_rows<I>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawOperatorRow>,
    {
        for (mdoc_opt, name_opt, start_opt, stop_opt) in raw_rows {
            let mdoc = match mdoc_opt {
                Some(id) if id > 0 => id,
                Some(id) => {
                    warn!("skip operator row: invalid mdoc <= 0: {id}");
                    continue;
                }
                None => {
                    warn!("skip operator row: mdoc was NULL");
                    continue;
                }
            };
            let name_raw = if let Some(s) = name_opt {
                s
            } else {
                warn!("skip operator row: bad UTF-8 name");
                continue;
            };
            let name_trimmed = name_raw.trim();
            if name_trimmed.is_empty() {
                warn!("skip operator row: empty name");
                continue;
            }
            // skip duplicates
            if let Ok(Some(_)) = self.deps.op_repo.get_by_mdoc(mdoc) {
                warn!("skip operator row: duplicate mdoc {mdoc}");
                continue;
            }
            // convert start
            let start = if let Some(ts) = start_opt {
                match Self::ts_to_naive(ts) {
                    Ok(dt) => dt,
                    Err(e) => {
                        warn!("skip operator row {mdoc}: bad start date: {e}");
                        continue;
                    }
                }
            } else {
                warn!("skip operator row {mdoc}: start NULL");
                continue;
            };
            // convert stop (skip bad stops, but allow None)
            let stop = match stop_opt {
                Some(ts) => match Self::ts_to_naive(ts) {
                    Ok(dt) => Some(dt),
                    Err(e) => {
                        warn!("skip operator row {mdoc}: bad stop date: {e}");
                        continue;
                    }
                },
                None => None,
            };

            let op = Operator {
                mdoc,
                name: name_trimmed.to_string(),
                start: Some(start),
                stop,
            };
            if let Err(e) = self.deps.op_repo.create(&op) {
                warn!("skip operator row {mdoc}: insert error: {e}");
                continue;
            }
        }
        Ok(())
    }

    // Migrate products from tblProduct
    pub fn migrate_products(&self, conn: &odbc_api::Connection<'_>) -> Result<(), AppError> {
        // collect raw rows: (upc, category, desc, price, updated, added, deleted)
        let mut raws: Vec<RawProductRow> = Vec::new();
        let mut cursor = conn
            .execute(
                "SELECT txtProductUPC, txtProductCategory, txtProductDescription, \
                curProductPrice, datUpdated, datAdded, datDeleted \
         FROM tblProduct",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblProduct".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            let mut upc_buf = Vec::new();
            let mut cat_buf = Vec::new();
            let mut desc_buf = Vec::new();
            let mut price_buf = Nullable::<f64>::null();
            let mut upd_buf = Nullable::<Timestamp>::null();
            let mut add_buf = Nullable::<Timestamp>::null();
            let mut del_buf = Nullable::<Timestamp>::null();

            let upc = match row.get_text(1, &mut upc_buf) {
                Ok(_) => Some(String::from_utf8_lossy(&upc_buf).to_string()),
                Err(e) => {
                    warn!("skip product row: bad UPC ({e})");
                    None
                }
            };

            let category = match row.get_text(2, &mut cat_buf) {
                Ok(_) => Some(String::from_utf8_lossy(&cat_buf).to_string()),
                Err(e) => {
                    warn!("skip product row: bad category ({e})");
                    None
                }
            };

            let desc = match row.get_text(3, &mut desc_buf) {
                Ok(_) => Some(String::from_utf8_lossy(&desc_buf).to_string()),
                Err(e) => {
                    warn!("skip product row: bad description ({e})");
                    None
                }
            };

            let price = match row.get_data(4, &mut price_buf) {
                Ok(()) => price_buf.into_opt(),
                Err(e) => {
                    warn!("skip product row: bad price ({e})");
                    None
                }
            };

            let updated = match row.get_data(5, &mut upd_buf) {
                Ok(()) => upd_buf.into_opt(),
                Err(e) => {
                    warn!("skip product row: bad updated ({e})");
                    None
                }
            };

            let added = match row.get_data(6, &mut add_buf) {
                Ok(()) => add_buf.into_opt(),
                Err(e) => {
                    warn!("skip product row: bad added ({e})");
                    None
                }
            };

            let deleted = match row.get_data(7, &mut del_buf) {
                Ok(()) => del_buf.into_opt(),
                Err(_) => None,
            };

            raws.push((upc, category, desc, price, updated, added, deleted));
        }
        self.migrate_products_from_rows(raws)
    }

    // Testable logic for products migrations
    pub(crate) fn migrate_products_from_rows<I>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawProductRow>,
    {
        for (upc_opt, cat_opt, desc_opt, price_opt, upd_opt, add_opt, del_opt) in raw_rows {
            let upc = if let Some(s) = upc_opt {
                let t = s.trim();
                if !(t.len() == 8 || t.len() == 12 || t.len() == 14)
                    || !t.chars().all(|c| c.is_ascii_digit())
                {
                    warn!("skip product row: invalid UPC '{t}'");
                    continue;
                }
                t.to_string()
            } else {
                warn!("skip product row: missing UPC");
                continue;
            };
            let category = if let Some(s) = cat_opt {
                let t = s.trim();
                if t.is_empty() {
                    warn!("skip product row: empty category");
                    continue;
                }
                t.to_string()
            } else {
                warn!("skip product row: missing category");
                continue;
            };
            let desc = if let Some(d) = desc_opt {
                let t = d.trim();
                if t.is_empty() {
                    warn!("skip product row: empty description");
                    continue;
                }
                t.to_string()
            } else {
                warn!("skip product row: missing description");
                continue;
            };
            let price_cents = if let Some(p) = price_opt {
                (p * 100.0).round() as i32
            } else {
                warn!("skip product row: missing price");
                continue;
            };
            if price_cents <= 0 {
                warn!("skip product row: zero or negative price {price_cents}");
                continue;
            }
            let updated = if let Some(ts) = upd_opt {
                match Self::ts_to_naive(ts) {
                    Ok(dt) => dt,
                    Err(e) => {
                        warn!("skip product {upc}: bad updated date: {e}");
                        continue;
                    }
                }
            } else {
                warn!("skip product {upc}: updated was NULL");
                continue;
            };
            let added = if let Some(ts) = add_opt {
                match Self::ts_to_naive(ts) {
                    Ok(dt) => dt,
                    Err(e) => {
                        warn!("skip product {upc}: bad added date: {e}");
                        continue;
                    }
                }
            } else {
                warn!("skip product {upc}: added was NULL");
                continue;
            };
            let deleted = match del_opt {
                Some(ts) => match Self::ts_to_naive(ts) {
                    Ok(dt) => Some(dt),
                    Err(e) => {
                        warn!("skip product {upc}: bad deleted date: {e}");
                        continue;
                    }
                },
                None => None,
            };

            let prod = Product {
                upc,
                category,
                desc,
                price: price_cents,
                updated: Some(updated),
                added: Some(added),
                deleted,
            };
            if let Err(e) = self.deps.product_repo.create(&prod) {
                warn!("skip product: insert error: {e}");
                continue;
            }
        }
        Ok(())
    }

    // Migrate categories from tblProduct
    pub fn migrate_categories(&self, conn: &odbc_api::Connection<'_>) -> Result<(), AppError> {
        // Build a Vec<Option<String>> of each category raw value
        let mut rows: Vec<RawCategoryRow<String>> = Vec::new();
        let mut cursor = conn
            .execute(
                "SELECT DISTINCT txtProductCategory FROM tblProduct",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblProduct".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            let mut buf = Vec::<u8>::new();
            if row.get_text(1, &mut buf).is_err() {
                warn!("skip category row: bad text");
                rows.push(None);
            } else {
                rows.push(Some(String::from_utf8_lossy(&buf).into_owned()));
            }
        }
        // Delegate to the iterator‐based implementation
        self.migrate_categories_from_rows(rows)
    }

    // Testable logic for categories migrations
    pub(crate) fn migrate_categories_from_rows<I, S>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawCategoryRow<S>>,
        S: AsRef<str>,
    {
        for opt in raw_rows {
            if let Some(s) = opt {
                let name = s.as_ref().trim();
                if name.is_empty() {
                    warn!("skip category row: empty name");
                    continue;
                }
                if let Err(e) = self.deps.category_repo.create(name.to_string()) {
                    warn!("skip category '{name}': insert error: {e}");
                    continue;
                }
            } else {
                warn!("skip category row: null buffer");
            }
        }
        Ok(())
    }

    // Migrate customers from tblCustomer
    pub fn migrate_customers(&self, conn: &odbc_api::Connection<'_>) -> Result<(), AppError> {
        // collect raw rows: (mdoc, name, added_timestamp)
        let mut raws: Vec<RawCustomerRow> = Vec::new();
        let mut mdoc_buf = Nullable::<i32>::null();
        let mut name_buf = Vec::<u8>::new();
        let mut added_buf = Nullable::<Timestamp>::null();
        let mut cursor = conn
            .execute(
                "SELECT lngCustomerMDOC, txtCustomerName, datAdded FROM tblCustomer",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblCustomer".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            if let Err(e) = row.get_data(1, &mut mdoc_buf) {
                warn!("skip customer row: bad mdoc: {e}");
                continue;
            }
            if let Err(e) = row.get_text(2, &mut name_buf) {
                warn!("skip customer row: bad name: {e}");
                continue;
            }
            // added timestamp
            if let Err(e) = row.get_data(3, &mut added_buf) {
                warn!("skip customer row: bad added date: {e}");
                continue;
            }

            let mdoc = mdoc_buf.into_opt();
            let added = added_buf.into_opt();
            let name = Some(String::from_utf8_lossy(&name_buf).into_owned());

            raws.push((mdoc, name, added));
        }
        self.migrate_customers_from_rows(raws)
    }

    // Testable logic for customer migrations
    pub(crate) fn migrate_customers_from_rows<I>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawCustomerRow>,
    {
        for (mdoc_opt, name_opt, added_opt) in raw_rows {
            let mdoc = match mdoc_opt {
                Some(id) if id > 0 => id,
                _ => {
                    warn!("skip customer row: invalid or missing mdoc");
                    continue;
                }
            };
            let name = if let Some(s) = name_opt {
                let t = s.trim();
                if t.is_empty() {
                    warn!("skip customer row: empty name");
                    continue;
                }
                t.to_string()
            } else {
                warn!("skip customer row: missing name");
                continue;
            };
            let dt = if let Some(ts) = added_opt {
                match Self::ts_to_naive(ts) {
                    Ok(dt) => dt,
                    Err(e) => {
                        warn!("skip customer row: invalid added date: {e}");
                        continue;
                    }
                }
            } else {
                warn!("skip customer row: missing added date");
                continue;
            };

            let cust = Customer {
                mdoc,
                name,
                added: dt,
                updated: dt,
            };
            if let Err(e) = self.deps.customer_repo.create(&cust) {
                warn!("skip customer row {mdoc}: insert error: {e}");
                continue;
            }
        }
        Ok(())
    }

    // Migrate club statements from tblClubStatement
    pub fn migrate_club_imports(&self, conn: &odbc_api::Connection<'_>) -> Result<(), AppError> {
        // collect raw rows: (id, activity, file, imported_ts)
        let mut raws: Vec<RawClubImportRow> = Vec::new();
        let mut cursor = conn
            .execute(
                "SELECT idnClubStatement, txtAccountActivity, txtFileName, datImported FROM tblClubStatement",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblClubStatement".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            let mut id_buf = Nullable::<i32>::null();
            let mut activity_buf = Vec::<u8>::new();
            let mut file_buf = Vec::<u8>::new();
            let mut imp_buf = Nullable::<Timestamp>::null();

            if let Err(e) = row.get_data(1, &mut id_buf) {
                warn!("skip club stmt row: bad id ({e})");
                continue;
            }
            if let Err(e) = row.get_text(2, &mut activity_buf) {
                warn!("skip club stmt row: bad activity ({e})");
                continue;
            }
            if let Err(e) = row.get_text(3, &mut file_buf) {
                warn!("skip club stmt row: bad file name ({e})");
                continue;
            }
            if let Err(e) = row.get_data(4, &mut imp_buf) {
                warn!("skip club stmt row: bad imported date ({e})");
                continue;
            }

            raws.push((
                id_buf.into_opt(),
                Some(String::from_utf8_lossy(&activity_buf).into_owned()),
                Some(String::from_utf8_lossy(&file_buf).into_owned()),
                imp_buf.into_opt(),
            ));
        }
        self.migrate_club_imports_from_rows(raws)
    }

    // Testable logic for ClubImport migrations
    pub(crate) fn migrate_club_imports_from_rows<I>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawClubImportRow>,
    {
        for (id_opt, act_opt, file_opt, imp_opt) in raw_rows {
            let id = match id_opt {
                Some(i) if i > 0 => i,
                _ => {
                    warn!("skip club stmt row: invalid or missing id");
                    continue;
                }
            };
            let act_str = if let Some(s) = act_opt {
                s
            } else {
                warn!("skip club stmt row {id}: missing activity");
                continue;
            };
            // split "MM/DD/YYYY - MM/DD/YYYY"
            let parts: Vec<&str> = act_str.split(" - ").collect();
            if parts.len() != 2 {
                warn!("skip club stmt row {id}: invalid activity '{act_str}'");
                continue;
            }
            let from_date = match chrono::NaiveDate::parse_from_str(parts[0].trim(), "%m/%d/%Y") {
                Ok(d) => d,
                Err(e) => {
                    warn!("skip club_import {id}: bad from date: {e}");
                    continue;
                }
            };
            let to_date = match chrono::NaiveDate::parse_from_str(parts[1].trim(), "%m/%d/%Y") {
                Ok(d) => d,
                Err(e) => {
                    warn!("skip club_import {id}: bad to date: {e}");
                    continue;
                }
            };
            let time = chrono::NaiveTime::from_hms_opt(0, 0, 0)
                .ok_or_else(|| AppError::Unexpected("failed to construct midnight time".into()))?;
            let activity_from = chrono::NaiveDateTime::new(from_date, time);
            let activity_to = chrono::NaiveDateTime::new(to_date, time);

            let source_file = if let Some(f) = file_opt {
                let t = f.trim();
                if t.is_empty() {
                    warn!("skip club stmt row {id}: empty source_file");
                    continue;
                }
                t.to_string()
            } else {
                warn!("skip club stmt row {id}: missing file name");
                continue;
            };

            let imp_ts = if let Some(ts) = imp_opt {
                ts
            } else {
                warn!("skip club stmt row {id}: missing imported date");
                continue;
            };
            let date = match Self::ts_to_naive(imp_ts) {
                Ok(dt) => dt,
                Err(e) => {
                    warn!("skip club_import {id}: bad imported ts: {e}");
                    continue;
                }
            };

            let stmt = ClubImport {
                id,
                date,
                activity_from,
                activity_to,
                source_file,
            };
            if let Err(e) = self.deps.club_imports_repo.create(&stmt) {
                warn!("skip club_import {id}: insert error: {e}");
                continue;
            }
        }
        Ok(())
    }

    // Migrate details from tblClubStatementDetail
    pub fn migrate_club_transactions(
        &self,
        conn: &odbc_api::Connection<'_>,
    ) -> Result<(), AppError> {
        let mut raws: Vec<RawClubTransactionRow> = Vec::new();
        let mut cursor = conn
            .execute(
                "SELECT idnRecord, nClubStatement, txtReceivedFrom, txtTransaction, curAmount, datPosted \
                 FROM tblClubStatementDetail",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblClubStatementDetail".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            let mut id_buf = Nullable::<i32>::null();
            let mut stmt_id_buf = Nullable::<i32>::null();
            let mut from_buf = Vec::<u8>::new();
            let mut tx_buf = Vec::<u8>::new();
            let mut amt_buf = Nullable::<f64>::null();
            let mut dt_buf = Nullable::<Timestamp>::null();

            if row.get_data(1, &mut id_buf).is_err() {
                warn!("skip detail: bad id");
                continue;
            }
            if row.get_data(2, &mut stmt_id_buf).is_err() {
                warn!("skip detail: bad import_id");
                continue;
            }
            if row.get_text(3, &mut from_buf).is_err() {
                warn!("skip detail: bad received_from");
                continue;
            }
            if row.get_text(4, &mut tx_buf).is_err() {
                warn!("skip detail: bad transaction");
                continue;
            }
            if row.get_data(5, &mut amt_buf).is_err() {
                warn!("skip detail: bad amount");
                continue;
            }
            if row.get_data(6, &mut dt_buf).is_err() {
                warn!("skip detail: bad date");
                continue;
            }

            raws.push((
                id_buf.into_opt(),
                stmt_id_buf.into_opt(),
                Some(String::from_utf8_lossy(&from_buf).into_owned()),
                Some(String::from_utf8_lossy(&tx_buf).into_owned()),
                amt_buf.into_opt(),
                dt_buf.into_opt(),
            ));
        }
        self.migrate_club_transactions_from_rows(raws)
    }

    // Testable logic for ClubTransaction migrations
    pub(crate) fn migrate_club_transactions_from_rows<I>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawClubTransactionRow>,
    {
        let re_paren = regex::Regex::new(r"\((\d+)\)")
            .map_err(|e| AppError::Unexpected(format!("invalid re_paren: {e}")))?;
        let re_wd = regex::Regex::new(r"^(.+?)\s+(\d+)\s+(.+)$")
            .map_err(|e| AppError::Unexpected(format!("invalid re_wd: {e}")))?;

        for (id_opt, imp_opt, received_opt, tx_opt, amt_opt, dt_opt) in raw_rows {
            let id = match id_opt {
                Some(i) if i > 0 => i,
                _ => {
                    warn!("skip detail: invalid id");
                    continue;
                }
            };
            let import_id = match imp_opt {
                Some(i) if i > 0 => i,
                _ => {
                    warn!("skip detail {id}: invalid import_id");
                    continue;
                }
            };
            let received = if let Some(s) = received_opt {
                s.trim().to_string()
            } else {
                warn!("skip detail {id}: missing received");
                continue;
            };
            let tx_type_str = if let Some(s) = tx_opt {
                s.trim().to_string()
            } else {
                warn!("skip detail {id}: missing tx type");
                continue;
            };
            let amount = ((amt_opt.unwrap_or_default()) * 100.0).round() as i32;

            // parse entity_name and optional mdoc
            let (entity_name, mdoc) = if tx_type_str == "Recd Client Donation/Dues" {
                let mdoc_val = re_paren
                    .captures(&received)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse().ok());
                if mdoc_val.is_none() {
                    warn!("skip detail {id}: invalid mdoc in '{received}'");
                    continue;
                }
                let name = received
                    .split_once(" (")
                    .map_or(received.as_str(), |(name, _)| name)
                    .trim()
                    .to_string();
                (name, mdoc_val)
            } else if tx_type_str == "W/D General" {
                if let Some(cap) = re_wd.captures(&received) {
                    let num = cap.get(2).and_then(|m| m.as_str().parse().ok());
                    if let Some(n) = num {
                        let ent = if let Some(m) = cap.get(3) {
                            m.as_str().trim().to_string()
                        } else {
                            warn!("skip detail {id}: regex capture group 3 missing");
                            continue;
                        };
                        (ent, Some(n))
                    } else {
                        let ent = if let Some(m) = cap.get(1) {
                            m.as_str().trim().to_string()
                        } else {
                            warn!("skip detail {id}: regex capture group 1 missing");
                            continue;
                        };
                        (ent, None)
                    }
                } else {
                    (received.clone(), None)
                }
            } else {
                warn!("skip detail {id}: unknown tx type '{tx_type_str}'");
                continue;
            };

            let tx_type = match tx_type_str.as_str() {
                "Recd Client Donation/Dues" => TransactionType::Deposit,
                "W/D General" => TransactionType::Withdrawal,
                _ => unreachable!(),
            };

            let date = if let Some(ts) = dt_opt {
                match Self::ts_to_naive(ts) {
                    Ok(dt) => dt,
                    Err(e) => {
                        warn!("skip detail {id}: bad date: {e}");
                        continue;
                    }
                }
            } else {
                warn!("skip detail {id}: missing date");
                continue;
            };

            let detail = ClubTransaction {
                id,
                import_id,
                entity_name,
                mdoc,
                tx_type,
                amount,
                date,
            };
            if let Err(e) = self.deps.club_transaction_repo.create(&detail) {
                warn!("skip detail {id}: insert error: {e}");
                continue;
            }
        }
        Ok(())
    }

    // Migrate inventory adjustments from qryProductInvDetail
    pub fn migrate_inventory_transactions(
        &self,
        conn: &odbc_api::Connection<'_>,
    ) -> Result<(), AppError> {
        // raw tuple: (order_id, refnum, note, upc, adjustment, posted_ts, cust_mdoc, op_mdoc)
        let mut raws: Vec<RawInventoryRow> = Vec::new();
        let mut cursor = conn
            .execute(
                "SELECT p.lngOrderId, p.txtRefNum, p.txtNote, p.txtProductUPC, \
                        p.lngInventoryAdjustment, p.datPosted, \
                        o.lngCustomerMdoc, o.lngOperatorMdoc \
                 FROM tblProductInventory AS p \
                 LEFT JOIN tblCustomerOrder AS o \
                   ON p.lngOrderId = o.idnOrderId",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from qryProductInvDetail".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            // buffers
            let mut order_buf = Nullable::<i32>::null();
            let mut ref_buf = Vec::<u8>::new();
            let mut note_buf = Vec::<u8>::new();
            let mut upc_buf = Vec::<u8>::new();
            let mut adj_buf = Nullable::<i32>::null();
            let mut dt_buf = Nullable::<Timestamp>::null();
            let mut cust_buf = Nullable::<i32>::null();
            let mut op_buf = Nullable::<i32>::null();

            if row.get_data(1, &mut order_buf).is_err() {
                warn!("skip inv row: bad order_id");
                continue;
            }
            if row.get_text(2, &mut ref_buf).is_err() {
                warn!("skip inv row: bad refnum");
                continue;
            }
            if row.get_text(3, &mut note_buf).is_err() {
                warn!("skip inv row: bad note");
                continue;
            }
            if row.get_text(4, &mut upc_buf).is_err() {
                warn!("skip inv row: bad upc");
                continue;
            }
            if row.get_data(5, &mut adj_buf).is_err() {
                warn!("skip inv row: bad adj");
                continue;
            }
            if row.get_data(6, &mut dt_buf).is_err() {
                warn!("skip inv row: bad posted date");
                continue;
            }
            if row.get_data(7, &mut cust_buf).is_err() {
                warn!("skip inv row: bad cust_mdoc");
                continue;
            }
            if row.get_data(8, &mut op_buf).is_err() {
                warn!("skip inv row: bad op_mdoc");
                continue;
            }

            raws.push((
                order_buf.into_opt(),
                Some(String::from_utf8_lossy(&ref_buf).into_owned()),
                Some(String::from_utf8_lossy(&note_buf).into_owned()),
                Some(String::from_utf8_lossy(&upc_buf).into_owned()),
                adj_buf.into_opt(),
                dt_buf.into_opt(),
                cust_buf.into_opt(),
                op_buf.into_opt(),
            ));
        }

        self.migrate_inventory_transactions_from_rows(raws)
    }

    // Testable logic for inventory migrations
    pub(crate) fn migrate_inventory_transactions_from_rows<I>(
        &self,
        raw_rows: I,
    ) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawInventoryRow>,
    {
        for (ord_opt, ref_opt, note_opt, upc_opt, adj_opt, ts_opt, cust_opt, op_opt) in raw_rows {
            // ref_order_id must be >0
            let ref_order_id = ord_opt.filter(|i| *i > 0);
            // build reference string
            let refnum = ref_opt.as_deref().unwrap_or("").trim();
            let note = note_opt.as_deref().unwrap_or("").trim();
            let reference = if !refnum.is_empty() && !note.is_empty() {
                Some(format!("{refnum} - {note}"))
            } else if !refnum.is_empty() {
                Some(refnum.to_string())
            } else if !note.is_empty() {
                Some(note.to_string())
            } else {
                None
            };
            // upc: must be 8, 12 or 14 digits
            let upc = upc_opt.unwrap_or_default().trim().to_string();
            if !((upc.len() == 8 || upc.len() == 12 || upc.len() == 14)
                && upc.chars().all(|c| c.is_ascii_digit()))
            {
                warn!("skip inv {ref_order_id:?}: invalid upc '{upc}'");
                continue;
            }
            // amount
            let quantity_change = adj_opt.unwrap_or_default();
            // skip if amount is 0
            if quantity_change == 0 {
                warn!("skip inv {ref_order_id:?}: zero quantity");
                continue;
            }
            // created_at
            let created_at = if let Some(ts) = ts_opt {
                Self::ts_to_naive(ts)?
            } else {
                warn!("skip inv {ref_order_id:?}: missing posted date");
                continue;
            };
            // operator_mdoc
            let operator_mdoc = op_opt.unwrap_or(0);
            // customer_mdoc optional
            let customer_mdoc = cust_opt.filter(|&i| i > 0);

            // finally persist
            let itx = InventoryTransaction {
                id: None,
                ref_order_id,
                reference,
                upc,
                quantity_change,
                created_at: Some(created_at),
                customer_mdoc,
                operator_mdoc,
            };
            if let Err(e) = self.deps.inv_repo.create(&itx) {
                warn!("skip inventory tx for UPC {}: insert error: {e}", itx.upc);
                continue;
            }
        }
        Ok(())
    }

    // Migrate customer orders from tblCustomerOrder
    pub fn migrate_customer_orders(&self, conn: &odbc_api::Connection<'_>) -> Result<(), AppError> {
        let mut raws: Vec<RawCustomerOrderRow> = Vec::new();

        // get data - filtering out orders that don't exist in tblCustomerAccount
        let mut cursor = conn
            .execute(
                "SELECT o.idnOrderId, o.lngCustomerMdoc, o.lngOperatorMdoc, o.datEntry, o.txtOrderNote \
                 FROM tblCustomerOrder o \
                 INNER JOIN (SELECT DISTINCT lngOrderId FROM tblCustomerAccount) a \
                   ON o.idnOrderId = a.lngOrderId",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| AppError::Unexpected("No result set from tblCustomerOrder".into()))?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            // idnOrderId (we don't actually use it in the new auto‑increment PK)
            let mut order_buf = Nullable::<i32>::null();
            let _ = row.get_data(1, &mut order_buf);

            // customer_mdoc
            let mut cust_buf = Nullable::<i32>::null();
            let cust_opt = match row.get_data(2, &mut cust_buf) {
                Ok(()) => cust_buf.into_opt(),
                Err(e) => {
                    warn!("skip order row: bad customer_mdoc: {e}");
                    continue;
                }
            };

            // operator_mdoc
            let mut op_buf = Nullable::<i32>::null();
            let op_opt = match row.get_data(3, &mut op_buf) {
                Ok(()) => op_buf.into_opt(),
                Err(e) => {
                    warn!("skip order row: bad operator_mdoc: {e}");
                    continue;
                }
            };

            // entry timestamp
            let mut dt_buf = Nullable::<Timestamp>::null();
            let dt_opt = match row.get_data(4, &mut dt_buf) {
                Ok(()) => dt_buf.into_opt(),
                Err(e) => {
                    warn!("skip order row: bad entry date: {e}");
                    continue;
                }
            };

            // note text
            let mut note_buf = Vec::<u8>::new();
            let note_opt = match row.get_text(5, &mut note_buf) {
                Ok(_) => Some(String::from_utf8_lossy(&note_buf).to_string()),
                Err(e) => {
                    warn!("skip order row: bad note text: {e}");
                    continue;
                }
            };

            raws.push((order_buf.into_opt(), cust_opt, op_opt, dt_opt, note_opt));
        }

        self.migrate_customer_orders_from_rows(raws)
    }

    // Testable logic: validate & insert each row
    pub(crate) fn migrate_customer_orders_from_rows<I>(&self, raw_rows: I) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawCustomerOrderRow>,
    {
        for (order_opt, cust_opt, op_opt, dt_opt, note_opt) in raw_rows {
            // customer_mdoc > 0
            let customer_mdoc = match cust_opt {
                Some(n) if n > 0 => n,
                Some(n) => {
                    warn!("skip customer_order: invalid customer_mdoc <= 0 ({n})");
                    continue;
                }
                None => {
                    warn!("skip customer_order: missing customer_mdoc");
                    continue;
                }
            };

            // operator_mdoc > 0
            let operator_mdoc = match op_opt {
                Some(n) if n > 0 => n,
                Some(n) => {
                    warn!("skip customer_order: invalid operator_mdoc <= 0 ({n})");
                    continue;
                }
                None => {
                    warn!("skip customer_order: missing operator_mdoc");
                    continue;
                }
            };

            // date required
            let date = if let Some(ts) = dt_opt {
                match Self::ts_to_naive(ts) {
                    Ok(dt) => dt,
                    Err(e) => {
                        warn!("skip customer_order: bad entry date: {e}");
                        continue;
                    }
                }
            } else {
                warn!("skip customer_order: missing entry date");
                continue;
            };

            // note: optional, empty→None
            let note = note_opt
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            // use the legacy order_id as the FK
            let order_id = match order_opt {
                Some(n) if n > 0 => n,
                Some(n) => {
                    warn!("skip customer_order: invalid order_id <= 0 ({n})");
                    continue;
                }
                None => {
                    warn!("skip customer_order: missing order_id");
                    continue;
                }
            };
            let tx = CustomerTransaction {
                order_id,
                customer_mdoc,
                operator_mdoc,
                date: Some(date),
                note,
            };

            if let Err(e) = self.deps.customer_transaction_repo.create(&tx) {
                warn!("skip customer_order {order_id}: insert error: {e}");
                continue;
            }
        }

        Ok(())
    }

    // Migrate customer order details from tblCustomerOrderDetail
    pub fn migrate_customer_order_details(
        &self,
        conn: &odbc_api::Connection<'_>,
    ) -> Result<(), AppError> {
        let mut raws: Vec<RawCustomerOrderDetailRow> = Vec::new();

        let mut cursor = conn
            .execute(
                "SELECT idnOrderDetailId, lngOrderId, txtProductUPC, lngQty, curProductPrice \
                 FROM tblCustomerOrderDetail",
                (),
                None::<usize>,
            )
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                AppError::Unexpected("No result set from tblCustomerOrderDetail".into())
            })?;

        while let Some(mut row) = cursor
            .next_row()
            .map_err(|e| AppError::Unexpected(e.to_string()))?
        {
            // legacy idnOrderDetailId (we'll ignore it)
            let mut detail_buf = Nullable::<i32>::null();
            let _ = row.get_data(1, &mut detail_buf);

            // order_id (foreign key)
            let mut order_buf = Nullable::<i32>::null();
            let order_opt = match row.get_data(2, &mut order_buf) {
                Ok(()) => order_buf.into_opt(),
                Err(e) => {
                    warn!("skip detail row: bad order_id: {e}");
                    continue;
                }
            };

            // upc
            let mut upc_buf = Vec::<u8>::new();
            let upc_opt = match row.get_text(3, &mut upc_buf) {
                Ok(_) => Some(String::from_utf8_lossy(&upc_buf).to_string()),
                Err(e) => {
                    warn!("skip detail row: bad UPC text: {e}");
                    continue;
                }
            };

            // quantity
            let mut qty_buf = Nullable::<i32>::null();
            let qty_opt = match row.get_data(4, &mut qty_buf) {
                Ok(()) => qty_buf.into_opt(),
                Err(e) => {
                    warn!("skip detail row: bad qty: {e}");
                    continue;
                }
            };

            // price
            let mut price_buf = Nullable::<f64>::null();
            let price_opt = match row.get_data(5, &mut price_buf) {
                Ok(()) => price_buf.into_opt(),
                Err(e) => {
                    warn!("skip detail row: bad price: {e}");
                    continue;
                }
            };

            raws.push((
                detail_buf.into_opt(),
                order_opt,
                upc_opt,
                qty_opt,
                price_opt,
            ));
        }

        self.migrate_customer_order_details_from_rows(raws)
    }

    // Testable logic for customer order details
    pub(crate) fn migrate_customer_order_details_from_rows<I>(
        &self,
        raw_rows: I,
    ) -> Result<(), AppError>
    where
        I: IntoIterator<Item = RawCustomerOrderDetailRow>,
    {
        for (_legacy_id, order_opt, upc_opt, qty_opt, price_opt) in raw_rows {
            // order_id > 0
            let order_id = match order_opt {
                Some(n) if n > 0 => n,
                Some(n) => {
                    warn!("skip detail: invalid order_id <= 0 ({n})");
                    continue;
                }
                None => {
                    warn!("skip detail: missing order_id");
                    continue;
                }
            };

            // upc: must be exactly 8, 12 or 14 numerical characters
            let upc = match upc_opt {
                Some(s)
                    if (s.trim().len() == 8 || s.trim().len() == 12 || s.trim().len() == 14)
                        && s.trim().chars().all(|c| c.is_ascii_digit()) =>
                {
                    s.trim().to_string()
                }
                _ => {
                    warn!("skip detail {order_id}: missing or bad UPC");
                    continue;
                }
            };

            // quantity exists
            let quantity = if let Some(n) = qty_opt {
                n
            } else {
                warn!("skip detail {order_id}: missing qty");
                continue;
            };

            // price must be > 0
            let price_cents = if let Some(p) = price_opt {
                let c = (p * 100.0).round() as i32;
                if c <= 0 {
                    warn!("skip detail {order_id}: non-positive price_cents ({c})");
                    continue;
                }
                c
            } else {
                warn!("skip detail {order_id}: missing price");
                continue;
            };

            let detail = CustomerTxDetail {
                detail_id: 0, // auto‐assigned PK
                order_id,
                upc,
                quantity,
                price: price_cents,
            };

            if let Err(e) = self.deps.cust_tx_detail_repo.create(&detail) {
                warn!("skip detail {order_id}: insert error: {e}");
                continue;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::mock_category_repo::MockCategoryRepo;
    use crate::test_support::mock_club_import_repo::MockClubImportRepo;
    use crate::test_support::mock_club_tx_repo::MockClubTransactionRepo;
    use crate::test_support::mock_customer_repo::MockCustomerRepo;
    use crate::test_support::mock_customer_tx_detail_repo::MockCustomerTxDetailRepo;
    use crate::test_support::mock_customer_tx_repo::MockCustomerTransactionRepo;
    use crate::test_support::mock_inventory_transaction_repo::MockInventoryTransactionRepo;
    use crate::test_support::mock_operator_repo::MockOperatorRepo;
    use crate::test_support::mock_product_repo::MockProductRepo;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use odbc_sys::Timestamp;
    use std::sync::Arc;

    impl Default for LegacyMigrationDeps {
        fn default() -> Self {
            Self {
                op_repo: Arc::new(MockOperatorRepo::new()),
                product_repo: Arc::new(MockProductRepo::new()),
                category_repo: Arc::new(MockCategoryRepo::new()),
                customer_repo: Arc::new(MockCustomerRepo::new()),
                club_transaction_repo: Arc::new(MockClubTransactionRepo::new()),
                club_imports_repo: Arc::new(MockClubImportRepo::new()),
                inv_repo: Arc::new(MockInventoryTransactionRepo::new()),
                customer_transaction_repo: Arc::new(MockCustomerTransactionRepo::new()),
                cust_tx_detail_repo: Arc::new(MockCustomerTxDetailRepo::new()),
                sqlite_conn: Arc::new(Mutex::new(rusqlite::Connection::open_in_memory().unwrap())),
            }
        }
    }

    fn make_usecase() -> LegacyMigrationUseCases {
        let deps = LegacyMigrationDeps::default();
        LegacyMigrationUseCases::new(deps)
    }

    // ts_to_naive tests
    #[test]
    fn ts_to_naive_valid_timestamp() {
        let ts = Timestamp {
            year: 2024,
            month: 7,
            day: 15,
            hour: 14,
            minute: 30,
            second: 45,
            fraction: 123_456,
        };
        let dt = LegacyMigrationUseCases::ts_to_naive(ts).expect("valid timestamp should parse");
        assert_eq!(dt.to_string(), "2024-07-15 14:30:45.123456");
    }

    #[test]
    fn ts_to_naive_invalid_date() {
        let ts = Timestamp {
            year: 2024,
            month: 13,
            day: 15,
            hour: 14,
            minute: 30,
            second: 45,
            fraction: 0,
        };
        let err = LegacyMigrationUseCases::ts_to_naive(ts).unwrap_err();
        assert!(err.to_string().contains("invalid date"));
    }

    #[test]
    fn ts_to_naive_invalid_time() {
        let ts = Timestamp {
            year: 2024,
            month: 7,
            day: 15,
            hour: 25,
            minute: 30,
            second: 45,
            fraction: 0,
        };
        let err = LegacyMigrationUseCases::ts_to_naive(ts).unwrap_err();
        assert!(err.to_string().contains("invalid time"));
    }

    // Category migration tests
    #[test]
    fn migrate_categories_from_rows_works() {
        let repo = Arc::new(MockCategoryRepo::new());
        let usecase = LegacyMigrationUseCases {
            deps: LegacyMigrationDeps {
                category_repo: repo.clone(),
                op_repo: Arc::new(MockOperatorRepo::new()),
                product_repo: Arc::new(MockProductRepo::new()),
                customer_repo: Arc::new(MockCustomerRepo::new()),
                club_transaction_repo: Arc::new(MockClubTransactionRepo::new()),
                club_imports_repo: Arc::new(MockClubImportRepo::new()),
                inv_repo: Arc::new(MockInventoryTransactionRepo::new()),
                customer_transaction_repo: Arc::new(MockCustomerTransactionRepo::new()),
                cust_tx_detail_repo: Arc::new(MockCustomerTxDetailRepo::new()),
                sqlite_conn: Arc::new(Mutex::new(rusqlite::Connection::open_in_memory().unwrap())),
            },
        };

        // simulate ODBC result: Some, whitespace, None
        let raw = vec![Some(" Snacks ".to_string()), Some("   ".to_string()), None];
        usecase
            .migrate_categories_from_rows(raw)
            .expect("should process rows");

        let stored = repo.list().unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].name, "Snacks");
    }

    // Operator migration tests
    #[test]
    fn operators_valid_row() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let rows = vec![(Some(1), Some("Alice".to_string()), Some(ts), None)];

        usecase
            .migrate_operators_from_rows(rows)
            .expect("valid row must succeed");

        let ops = usecase.deps.op_repo.list().unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].mdoc, 1);
        assert_eq!(ops[0].name, "Alice");
        // timestamp conversion
        let expected_start = LegacyMigrationUseCases::ts_to_naive(ts).unwrap();
        assert_eq!(ops[0].start.unwrap(), expected_start);
        assert!(ops[0].stop.is_none());
    }

    #[test]
    fn operators_skip_bad_mdoc() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let rows = vec![(None, Some("Bob".to_string()), Some(ts), None)];

        usecase
            .migrate_operators_from_rows(rows)
            .expect("bad mdoc should be skipped");

        let ops = usecase.deps.op_repo.list().unwrap();
        assert!(ops.is_empty());
    }

    #[test]
    fn operators_skip_empty_name() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let rows = vec![(Some(2), Some("   ".to_string()), Some(ts), None)];

        usecase
            .migrate_operators_from_rows(rows)
            .expect("empty name should be skipped");

        let ops = usecase.deps.op_repo.list().unwrap();
        assert!(ops.is_empty());
    }

    #[test]
    fn operators_skip_duplicate_mdoc() {
        let usecase = make_usecase();
        // pre-insert an operator with mdoc = 3
        let ts0 = Timestamp {
            year: 2025,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let existing = Operator {
            mdoc: 3,
            name: "Orig".into(),
            start: Some(LegacyMigrationUseCases::ts_to_naive(ts0).unwrap()),
            stop: None,
        };
        usecase.deps.op_repo.create(&existing).unwrap();

        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let rows = vec![(Some(3), Some("Clone".to_string()), Some(ts), None)];

        usecase
            .migrate_operators_from_rows(rows)
            .expect("duplicate mdoc should be skipped");

        let ops = usecase.deps.op_repo.list().unwrap();
        // still only the original operator
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].mdoc, 3);
        assert_eq!(ops[0].name, "Orig");
    }

    // Product migration tests
    #[test]
    fn products_valid_row_inserts() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let raw = vec![(
            Some("12345678".to_string()),
            Some("Cat".to_string()),
            Some("Desc".to_string()),
            Some(1.23),
            Some(ts),
            Some(ts),
            Some(ts),
        )];
        usecase.migrate_products_from_rows(raw).unwrap();
        let prods = usecase.deps.product_repo.list().unwrap();
        assert_eq!(prods.len(), 1);
        let p = &prods[0];
        assert_eq!(p.upc, "12345678");
        assert_eq!(p.category, "Cat");
        assert_eq!(p.desc, "Desc");
        assert_eq!(p.price, 123); // 1.23 * 100
        let dt = LegacyMigrationUseCases::ts_to_naive(ts).unwrap();
        assert_eq!(p.updated.unwrap(), dt);
        assert_eq!(p.added.unwrap(), dt);
        assert_eq!(p.deleted.unwrap(), dt);
    }

    #[test]
    fn products_skip_invalid_upc() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let raws = vec![
            // wrong length
            (
                Some("123".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some(1.0),
                Some(ts),
                Some(ts),
                None,
            ),
            // non-digit
            (
                Some("1234ABCD".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some(1.0),
                Some(ts),
                Some(ts),
                None,
            ),
        ];
        usecase.migrate_products_from_rows(raws).unwrap();
        assert!(usecase.deps.product_repo.list().unwrap().is_empty());
    }

    #[test]
    fn products_skip_empty_category_or_description() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let raws = vec![
            // empty category
            (
                Some("12345678".to_string()),
                Some("   ".to_string()),
                Some("D".to_string()),
                Some(1.0),
                Some(ts),
                Some(ts),
                None,
            ),
            // empty description
            (
                Some("12345678".to_string()),
                Some("C".to_string()),
                Some("   ".to_string()),
                Some(1.0),
                Some(ts),
                Some(ts),
                None,
            ),
        ];
        usecase.migrate_products_from_rows(raws).unwrap();
        assert!(usecase.deps.product_repo.list().unwrap().is_empty());
    }

    #[test]
    fn products_skip_zero_or_negative_price() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let raws = vec![
            (
                Some("12345678".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some(0.0),
                Some(ts),
                Some(ts),
                None,
            ),
            (
                Some("12345678".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some(-5.0),
                Some(ts),
                Some(ts),
                None,
            ),
        ];
        usecase.migrate_products_from_rows(raws).unwrap();
        assert!(usecase.deps.product_repo.list().unwrap().is_empty());
    }

    #[test]
    fn products_skip_missing_updated_or_added() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let raws = vec![
            // missing updated
            (
                Some("12345678".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some(1.0),
                None,
                Some(ts),
                None,
            ),
            // missing added
            (
                Some("12345678".to_string()),
                Some("C".to_string()),
                Some("D".to_string()),
                Some(1.0),
                Some(ts),
                None,
                None,
            ),
        ];
        usecase.migrate_products_from_rows(raws).unwrap();
        assert!(usecase.deps.product_repo.list().unwrap().is_empty());
    }

    // Customer migration tests
    #[test]
    fn customers_valid_row() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 6,
            day: 7,
            hour: 8,
            minute: 9,
            second: 10,
            fraction: 0,
        };
        let raw = vec![(Some(42), Some("Alice".to_string()), Some(ts))];

        usecase
            .migrate_customers_from_rows(raw)
            .expect("valid row should succeed");

        let list = usecase.deps.customer_repo.list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].mdoc, 42);
        assert_eq!(list[0].name, "Alice");
        let dt = LegacyMigrationUseCases::ts_to_naive(ts).unwrap();
        assert_eq!(list[0].added, dt);
        assert_eq!(list[0].updated, dt);
    }

    #[test]
    fn customers_skip_bad_mdoc() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 6,
            day: 7,
            hour: 8,
            minute: 9,
            second: 10,
            fraction: 0,
        };
        let raws = vec![
            (None, Some("Bob".to_string()), Some(ts)), // missing mdoc
            (Some(0), Some("Carol".to_string()), Some(ts)), // zero mdoc
        ];

        usecase
            .migrate_customers_from_rows(raws)
            .expect("bad mdoc should be skipped");

        assert!(usecase.deps.customer_repo.list().unwrap().is_empty());
    }

    #[test]
    fn customers_skip_empty_name_and_trim() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 6,
            day: 7,
            hour: 8,
            minute: 9,
            second: 10,
            fraction: 0,
        };
        let raws = vec![
            (Some(5), Some("   ".to_string()), Some(ts)), // empty after trim
            (Some(6), Some("  Dave  ".to_string()), Some(ts)), // trim whitespace
        ];

        usecase
            .migrate_customers_from_rows(raws)
            .expect("rows should be processed");

        let list = usecase.deps.customer_repo.list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].mdoc, 6);
        assert_eq!(list[0].name, "Dave");
    }

    #[test]
    fn customers_skip_missing_added_date() {
        let usecase = make_usecase();
        let raws = vec![
            (Some(7), Some("Eve".to_string()), None), // missing added
        ];

        usecase
            .migrate_customers_from_rows(raws)
            .expect("missing date should be skipped");

        assert!(usecase.deps.customer_repo.list().unwrap().is_empty());
    }

    // Club imports migration tests
    #[test]
    fn club_imports_valid_row() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
            minute: 4,
            second: 5,
            fraction: 0,
        };
        let raw = vec![(
            Some(10),
            Some("01/02/2025 - 03/04/2025".to_string()),
            Some(" file.txt ".to_string()),
            Some(ts),
        )];

        usecase
            .migrate_club_imports_from_rows(raw)
            .expect("valid row should succeed");

        let stored = usecase.deps.club_imports_repo.list().unwrap();
        assert_eq!(stored.len(), 1);
        let stmt = &stored[0];
        assert_eq!(stmt.id, 10);
        // activity range parsing
        let start = NaiveDate::from_ymd_opt(2025, 1, 2)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let end = NaiveDate::from_ymd_opt(2025, 3, 4)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        assert_eq!(stmt.activity_from, start);
        assert_eq!(stmt.activity_to, end);
        // source_file trimmed
        assert_eq!(stmt.source_file, "file.txt");
        // imported date
        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
            NaiveTime::from_hms_opt(3, 4, 5).unwrap(),
        );
        assert_eq!(stmt.date, dt);
    }

    #[test]
    fn club_imports_skip_bad_id() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![
            (
                None,
                Some("01/02/2025 - 03/04/2025".to_string()),
                Some("f".to_string()),
                Some(ts),
            ),
            (
                Some(0),
                Some("01/02/2025 - 03/04/2025".to_string()),
                Some("f".to_string()),
                Some(ts),
            ),
        ];

        usecase
            .migrate_club_imports_from_rows(raws)
            .expect("bad id should be skipped");

        assert!(usecase.deps.club_imports_repo.list().unwrap().is_empty());
    }

    #[test]
    fn club_imports_skip_missing_activity_or_format() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![
            (Some(1), None, Some("f".to_string()), Some(ts)),
            (
                Some(2),
                Some("bad-format".to_string()),
                Some("f".to_string()),
                Some(ts),
            ),
        ];

        usecase
            .migrate_club_imports_from_rows(raws)
            .expect("bad activity rows skipped");

        assert!(usecase.deps.club_imports_repo.list().unwrap().is_empty());
    }

    #[test]
    fn club_imports_skip_empty_file() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 2,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![(
            Some(3),
            Some("01/02/2025 - 03/04/2025".to_string()),
            Some("  ".to_string()),
            Some(ts),
        )];

        usecase
            .migrate_club_imports_from_rows(raws)
            .expect("empty file skipped");

        assert!(usecase.deps.club_imports_repo.list().unwrap().is_empty());
    }

    #[test]
    fn club_imports_skip_missing_import_date() {
        let usecase = make_usecase();
        let raws = vec![(
            Some(4),
            Some("01/02/2025 - 03/04/2025".to_string()),
            Some("f".to_string()),
            None,
        )];

        usecase
            .migrate_club_imports_from_rows(raws)
            .expect("missing import date skipped");

        assert!(usecase.deps.club_imports_repo.list().unwrap().is_empty());
    }

    // Club transaction migration tests
    #[test]
    fn deposits_with_valid_mdoc() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 8,
            hour: 9,
            minute: 10,
            second: 11,
            fraction: 0,
        };
        let raws = vec![(
            Some(1),
            Some(100),
            Some("  John Doe (123)  ".into()),
            Some("Recd Client Donation/Dues".into()),
            Some(12.34),
            Some(ts),
        )];

        usecase.migrate_club_transactions_from_rows(raws).unwrap();
        let stored = usecase.deps.club_transaction_repo.list().unwrap();
        assert_eq!(stored.len(), 1);
        let d = &stored[0];
        assert_eq!(d.id, 1);
        assert_eq!(d.import_id, 100);
        assert_eq!(d.entity_name, "John Doe");
        assert_eq!(d.mdoc, Some(123));
        assert_eq!(d.tx_type, TransactionType::Deposit);
        assert_eq!(d.amount, (12.34 * 100.0) as i32);
        let dt = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(9, 10, 11).unwrap());
        assert_eq!(d.date, dt);
    }

    #[test]
    fn skip_deposit_with_bad_mdoc() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 8,
            hour: 9,
            minute: 10,
            second: 11,
            fraction: 0,
        };
        let raws = vec![(
            Some(2),
            Some(200),
            Some("Foo Bar".into()),
            Some("Recd Client Donation/Dues".into()),
            Some(5.0),
            Some(ts),
        )];

        usecase.migrate_club_transactions_from_rows(raws).unwrap();
        assert!(usecase
            .deps
            .club_transaction_repo
            .list()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn withdrawals_with_and_without_mdoc() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 9,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![
            (
                Some(3),
                Some(300),
                Some("Facility 170308 Smith, Alice".into()),
                Some("W/D General".into()),
                Some(7.5),
                Some(ts),
            ),
            (
                Some(4),
                Some(400),
                Some("Just Facility".into()),
                Some("W/D General".into()),
                Some(3.25),
                Some(ts),
            ),
        ];

        usecase.migrate_club_transactions_from_rows(raws).unwrap();
        let list = usecase.deps.club_transaction_repo.list().unwrap();
        assert_eq!(list.len(), 2);
        let first = &list[0];
        assert_eq!(first.id, 3);
        assert_eq!(first.entity_name, "Smith, Alice");
        assert_eq!(first.mdoc, Some(170308));
        assert_eq!(first.tx_type, TransactionType::Withdrawal);
        assert_eq!(first.amount, (7.5 * 100.0) as i32);

        let second = &list[1];
        assert_eq!(second.id, 4);
        assert_eq!(second.entity_name, "Just Facility");
        assert_eq!(second.mdoc, None);
        assert_eq!(second.tx_type, TransactionType::Withdrawal);
    }

    #[test]
    fn skip_unknown_type_or_missing_date() {
        let usecase = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 9,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![
            // unknown tx type
            (
                Some(5),
                Some(500),
                Some("X".into()),
                Some("Unknown Type".into()),
                Some(1.0),
                Some(ts),
            ),
            // missing date
            (
                Some(6),
                Some(600),
                Some("Z (999)".into()),
                Some("Recd Client Donation/Dues".into()),
                Some(2.0),
                None,
            ),
        ];

        usecase.migrate_club_transactions_from_rows(raws).unwrap();
        assert!(usecase
            .deps
            .club_transaction_repo
            .list()
            .unwrap()
            .is_empty());
    }

    // Inventory transaction migration tests
    #[test]
    fn inv_full_valid_row() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 8,
            hour: 9,
            minute: 10,
            second: 11,
            fraction: 0,
        };
        let raws = vec![(
            Some(42),
            Some("R1".into()),
            Some("N1".into()),
            Some("000012345678".into()),
            Some(3),
            Some(ts),
            Some(7),
            Some(8),
        )];

        uc.migrate_inventory_transactions_from_rows(raws).unwrap();
        let stored = uc.deps.inv_repo.list().unwrap();
        assert_eq!(stored.len(), 1);
        let tx = &stored[0];
        assert_eq!(tx.ref_order_id, Some(42));
        assert_eq!(tx.reference.as_ref().unwrap(), "R1 - N1");
        assert_eq!(tx.upc, "000012345678");
        assert_eq!(tx.quantity_change, 3);
        let dt = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(9, 10, 11).unwrap());
        assert_eq!(tx.created_at, Some(dt));
        assert_eq!(tx.customer_mdoc, Some(7));
        assert_eq!(tx.operator_mdoc, 8);
    }

    #[test]
    fn inv_optional_ref_and_note() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![
            // only refnum
            (
                None,
                Some("R2".into()),
                Some("".into()),
                Some("12345678".into()),
                Some(1),
                Some(ts),
                None,
                Some(5),
            ),
            // only note
            (
                None,
                Some("".into()),
                Some("N2".into()),
                Some("87654321".into()),
                Some(2),
                Some(ts),
                Some(9),
                Some(6),
            ),
            // neither -> reference None
            (
                None,
                Some("".into()),
                Some("".into()),
                Some("11223344".into()),
                Some(3),
                Some(ts),
                None,
                Some(7),
            ),
        ];

        uc.migrate_inventory_transactions_from_rows(raws).unwrap();
        let list = uc.deps.inv_repo.list().unwrap();
        assert_eq!(list.len(), 3);

        assert_eq!(list[0].reference.as_ref().unwrap(), "R2");
        assert_eq!(list[1].reference.as_ref().unwrap(), "N2");
        assert!(list[2].reference.is_none());
    }

    #[test]
    fn inv_skip_invalid_upc_or_amount() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 2,
            day: 2,
            hour: 2,
            minute: 2,
            second: 2,
            fraction: 0,
        };
        let raws = vec![
            // bad UPC
            (
                Some(1),
                Some("R".into()),
                Some("N".into()),
                Some("BADUPCLEN".into()),
                Some(1),
                Some(ts),
                Some(1),
                Some(1),
            ),
            // zero amount
            (
                Some(2),
                Some("R".into()),
                Some("N".into()),
                Some("12345678".into()),
                Some(0),
                Some(ts),
                Some(1),
                Some(1),
            ),
        ];

        uc.migrate_inventory_transactions_from_rows(raws).unwrap();
        assert!(uc.deps.inv_repo.list().unwrap().is_empty());
    }

    #[test]
    fn inv_skip_missing_date_or_operator() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 3,
            day: 3,
            hour: 3,
            minute: 3,
            second: 3,
            fraction: 0,
        };
        let raws = vec![
            // missing date
            (
                Some(1),
                Some("R".into()),
                Some("N".into()),
                Some("12345678".into()),
                Some(1),
                None,
                Some(2),
                Some(2),
            ),
            // missing operator
            (
                Some(2),
                Some("R".into()),
                Some("N".into()),
                Some("12345678".into()),
                Some(1),
                Some(ts),
                Some(2),
                None,
            ),
        ];

        uc.migrate_inventory_transactions_from_rows(raws).unwrap();
        let txs = uc.deps.inv_repo.list().unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].ref_order_id, Some(2));
        assert_eq!(txs[0].operator_mdoc, 0);
    }

    // Customer order migration tests
    #[test]
    fn orders_happy_path_and_note_variants() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 15,
            hour: 12,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        // RawCustomerOrderRow = (Option<i32>,Option<i32>,Option<i32>,Option<Timestamp>,Option<String>)
        let raws = vec![
            // all present, non‑empty note
            (
                Some(10),
                Some(1),
                Some(2),
                Some(ts.clone()),
                Some(" hello ".into()),
            ),
            // empty note → None
            (
                Some(11),
                Some(3),
                Some(4),
                Some(ts.clone()),
                Some("   ".into()),
            ),
        ];
        uc.migrate_customer_orders_from_rows(raws).unwrap();
        let saved = uc.deps.customer_transaction_repo.list().unwrap();
        assert_eq!(saved.len(), 2);
        // first
        assert_eq!(saved[0].order_id, 10);
        assert_eq!(saved[0].customer_mdoc, 1);
        assert_eq!(saved[0].operator_mdoc, 2);
        assert_eq!(saved[0].date.unwrap().to_string(), "2025-07-15 12:00:00");
        assert_eq!(saved[0].note.as_ref().unwrap(), "hello");
        // second: note None
        assert_eq!(saved[1].order_id, 11);
        assert!(saved[1].note.is_none());
    }
    #[test]
    fn orders_skip_missing_or_invalid_fields() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 15,
            hour: 12,
            minute: 0,
            second: 0,
            fraction: 0,
        };
        let raws = vec![
            // missing customer_mdoc
            (Some(1), None, Some(2), Some(ts.clone()), None),
            // invalid operator_mdoc <= 0
            (Some(2), Some(1), Some(0), Some(ts.clone()), None),
            // missing date
            (Some(3), Some(1), Some(2), None, None),
        ];
        uc.migrate_customer_orders_from_rows(raws).unwrap();
        assert!(uc.deps.customer_transaction_repo.list().unwrap().is_empty());
    }
    // Customer order detail migration tests
    #[test]
    fn migrate_customer_orders_creates_only_valid() {
        let uc = make_usecase();
        let ts = Timestamp {
            year: 2025,
            month: 7,
            day: 15,
            hour: 9,
            minute: 30,
            second: 0,
            fraction: 0,
        };
        // RawCustomerOrderRow = (Option<i32>, Option<i32>, Option<i32>, Option<Timestamp>, Option<String>)
        let raws = vec![
            // valid
            (Some(10), Some(1), Some(2), Some(ts), Some(" note ".into())),
            // missing order_id
            (None, Some(1), Some(2), Some(ts), Some("x".into())),
            // invalid cust_mdoc
            (Some(11), Some(0), Some(2), Some(ts), Some("x".into())),
            // missing op_mdoc
            (Some(12), Some(1), None, Some(ts), Some("x".into())),
            // missing date
            (Some(13), Some(1), Some(2), None, Some("x".into())),
            // empty note => None
            (Some(14), Some(3), Some(4), Some(ts), Some("   ".into())),
        ];

        uc.migrate_customer_orders_from_rows(raws).unwrap();
        let list = uc.deps.customer_transaction_repo.list().unwrap();
        // only two survived: the first valid, and the last with empty note
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].order_id, 10);
        assert_eq!(list[0].customer_mdoc, 1);
        assert_eq!(list[0].operator_mdoc, 2);
        assert_eq!(
            list[0].date,
            Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
                NaiveTime::from_hms_opt(9, 30, 0).unwrap()
            ))
        );
        assert_eq!(list[0].note.as_deref(), Some("note"));
        assert_eq!(list[1].order_id, 14);
        assert_eq!(list[1].note, None);
    }

    #[test]
    fn migrate_customer_order_details_creates_only_valid() {
        let uc = make_usecase();
        let raws = vec![
            // valid row: order_id=5, upc len8 digits, qty=0, price=$1.23
            (
                Some(1),
                Some(5),
                Some("12345678".into()),
                Some(0),
                Some(1.23),
            ),
            // bad order_id
            (
                Some(2),
                Some(0),
                Some("12345678".into()),
                Some(1),
                Some(1.23),
            ),
            // bad upc
            (Some(3), Some(6), Some("ABC".into()), Some(1), Some(1.23)),
            // negative qty
            (
                Some(4),
                Some(7),
                Some("87654321".into()),
                Some(-1),
                Some(1.23),
            ),
            // zero price
            (
                Some(5),
                Some(8),
                Some("87654321".into()),
                Some(1),
                Some(0.0),
            ),
        ];

        uc.migrate_customer_order_details_from_rows(raws).unwrap();

        // Only the valid row with order_id = 5 should be in the repo:
        let list = uc.deps.cust_tx_detail_repo.list_by_order(5).unwrap();

        assert_eq!(list.len(), 1);
        let det = &list[0].0;
        assert_eq!(det.order_id, 5);
        assert_eq!(det.upc, "12345678");
        assert_eq!(det.quantity, 0);
        assert_eq!(det.price, 123); // 1.23 * 100
    }
}
