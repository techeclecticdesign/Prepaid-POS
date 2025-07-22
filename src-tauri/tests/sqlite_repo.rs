use prepaid_pos_lib::domain::models::Operator;
use prepaid_pos_lib::domain::repos::OperatorRepoTrait;
use prepaid_pos_lib::infrastructure::repos::SqliteOperatorRepo;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

fn make_repo() -> SqliteOperatorRepo {
    // open an in-memory DB and create the table
    let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
    conn.lock()
        .unwrap()
        .execute_batch(
            "CREATE TABLE operators (
            mdoc   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            start TEXT NOT NULL,
            stop  TEXT
        )",
        )
        .unwrap();
    SqliteOperatorRepo::new(conn)
}

#[test]
fn sqlite_repo_crud() {
    let repo = make_repo();

    let op = Operator {
        mdoc: 1,
        name: "Test".into(),
        start: Some(chrono::Utc::now().naive_utc()),
        stop: None,
    };
    repo.create(&op).unwrap();

    let got = repo.get_by_mdoc(1).unwrap().unwrap();
    assert_eq!(got.name, "Test");

    let mut upd = got.clone();
    upd.name = "Test 2".into();
    repo.update_by_mdoc(&upd).unwrap();
    assert_eq!(repo.get_by_mdoc(1).unwrap().unwrap().name, "Test 2");

    let all = repo.list().unwrap();
    assert_eq!(all.len(), 1);
}
