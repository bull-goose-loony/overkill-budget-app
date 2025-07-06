
CREATE TABLE IF NOT EXISTS financial_record (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    record_type TEXT NOT NULL,
    frequency TEXT NOT NULL
    );

INSERT INTO financial_record (id, name, amount, record_type, frequency)
VALUES
  ('67e55044-10b1-426f-9247-bb680e5fe0c0', 'salary', 10000, 'Income', 'Montly'),
  ('67e55044-10b1-426f-9247-bb680e5fe0c1', 'Electric Bill', 127.33, 'Expense', 'Montly'),
  ('67e55044-10b1-426f-9247-bb680e5fe0c2', 'Gambling', 50000000, 'Income', 'Daily'),
  ('67e55044-10b1-426f-9247-bb680e5fe0c3', 'Trash Service', 120.23, 'Expense', 'Quarterly')
ON CONFLICT DO NOTHING;
    
