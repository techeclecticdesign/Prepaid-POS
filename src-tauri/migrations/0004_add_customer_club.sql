CREATE TABLE customer (
  mdoc    INTEGER PRIMARY KEY,   
  name    TEXT NOT NULL,
  added   DATETIME NOT NULL,
  updated DATETIME NOT NULL
);

CREATE TABLE club_transactions (
  id       INTEGER PRIMARY KEY AUTOINCREMENT,
  import_id INTEGER NOT NULL,
  entity_name TEXT,
  mdoc     INTEGER,                  
  tx_type  TEXT NOT NULL,            
  amount   INTEGER NOT NULL,
  date     DATETIME NOT NULL,
  FOREIGN KEY(mdoc) REFERENCES customer(mdoc)
);

CREATE TABLE club_imports (
  id             INTEGER PRIMARY KEY AUTOINCREMENT,
  date           DATETIME NOT NULL,
  activity_from  DATETIME NOT NULL,
  activity_to    DATETIME NOT NULL,
  source_file    TEXT NOT NULL
);
