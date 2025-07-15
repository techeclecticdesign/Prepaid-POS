CREATE TABLE IF NOT EXISTS customer_transactions (
    order_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_mdoc  INTEGER NOT NULL,
    operator_mdoc  INTEGER NOT NULL,
    date           TEXT,
    note           TEXT
);

CREATE TABLE IF NOT EXISTS customer_tx_detail (
    detail_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id    INTEGER NOT NULL,
    upc         TEXT    NOT NULL,
    quantity    INTEGER NOT NULL,
    price       INTEGER NOT NULL,

    FOREIGN KEY(order_id)
        REFERENCES customer_transactions(order_id)
        ON DELETE CASCADE
);
