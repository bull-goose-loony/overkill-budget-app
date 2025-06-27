
CREATE TABLE IF NOT EXISTS financial_record (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    record_type TEXT NOT NULL,
    frequency TEXT NOT NULL
);
