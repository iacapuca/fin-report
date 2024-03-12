CREATE TABLE transactions (
    id TEXT PRIMARY KEY,
    product_id TEXT,
    amount REAL NOT NULL,
    description TEXT,
    date TEXT NOT NULL,
    transaction_type TEXT NOT NULL CHECK (transaction_type IN ('sale', 'buy')),
    FOREIGN KEY (product_id) REFERENCES products(id)
);
