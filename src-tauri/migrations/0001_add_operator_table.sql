PRAGMA foreign_keys = ON;
PRAGMA foreign_key_check;
CREATE TABLE IF NOT EXISTS operators (
  mdoc    INTEGER PRIMARY KEY,
  name  TEXT NOT NULL,
  start TEXT NOT NULL,
  stop  TEXT
);
