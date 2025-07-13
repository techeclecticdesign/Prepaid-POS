CREATE TABLE products (
  upc     TEXT PRIMARY KEY,
  desc    TEXT NOT NULL,
  category TEXT NOT NULL,
  price   INTEGER NOT NULL,
  updated DATETIME NOT NULL,
  added   DATETIME NOT NULL,
  deleted DATETIME
);

CREATE TABLE inventory_transactions (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  upc               TEXT NOT NULL,     
  quantity_change   INTEGER NOT NULL,
  operator_mdoc     INTEGER NOT NULL,
  customer_mdoc     INTEGER,          
  ref_order_id      INTEGER,                   
  reference         TEXT,                      
  created_at        DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(upc) REFERENCES products(upc),
  FOREIGN KEY (operator_mdoc) REFERENCES operators(id)
);

CREATE TABLE price_adjustments (
  id   INTEGER PRIMARY KEY AUTOINCREMENT,
  operator_mdoc INTEGER NOT NULL,
  upc  TEXT NOT NULL,
  old  INTEGER NOT NULL,
  new  INTEGER NOT NULL,
  created_at        DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(upc) REFERENCES products(upc),
  FOREIGN KEY (operator_mdoc) REFERENCES operators(id)
);
