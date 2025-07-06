use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::sync::{LazyLock, Mutex};

// Lazily embed and initialize migrations
static MIGRATIONS: LazyLock<Migrations> = LazyLock::new(|| {
    Migrations::new(vec![
        M::up(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/migrations/0001_add_operator_table.sql"
        ))),
        M::up(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/migrations/0002_add_product_related_tables.sql"
        ))),
    ])
});

// Creates and configures a single shared SQLite connection.
pub fn create_connection(db_path: &str) -> anyhow::Result<Mutex<Connection>> {
    let mut conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    MIGRATIONS.to_latest(&mut conn)?;
    Ok(Mutex::new(conn))
}

// Tests for our `create_connection` helper and embedded migrations
#[cfg(test)]
mod tests {
    use super::*;

    // Verifies migrations applied (operators table exists)
    // and that foreign_keys pragma is turned on.
    #[test]
    fn create_connection_runs_migrations_and_pragmas() -> anyhow::Result<()> {
        // Use in-memory DB so tests are isolated
        let mutex = create_connection(":memory:")?;
        let conn = &mut *mutex.lock().unwrap();

        // Check that the "operators" table is present
        let table_count: i32 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='operators'",
            [],
            |r| r.get(0),
        )?;
        assert_eq!(
            table_count, 1,
            "operators table should exist after migration"
        );

        // Check that PRAGMA foreign_keys = ON
        let fk_enabled: i32 = conn.query_row("PRAGMA foreign_keys", [], |r| r.get(0))?;
        assert_eq!(fk_enabled, 1, "foreign_keys pragma should be enabled");

        Ok(())
    }
}
