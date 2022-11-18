-- Your SQL goes here

CREATE TABLE swaps (
    uuid TEXT PRIMARY KEY,
    amount INTEGER NOT NULL,
    payment_request TEXT NOT NULL,
    txid TEXT,
    status TEXT NOT NULL
);