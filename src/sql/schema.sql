
CREATE TABLE IF NOT EXISTS financial_record (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    record_type TEXT NOT NULL,
    frequency TEXT NOT NULL
);

-- Optional static data
INSERT INTO financial_record (id, name, amount, record_type, frequency)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'Paycheck',
    123.45,
    'Income',
    'Monthly'
);
