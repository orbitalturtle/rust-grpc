-- CREATE DATABASE towers OWNER you;
-- \c towers;

CREATE TABLE towers (
    id VARCHAR(255) PRIMARY KEY NOT NULL
);

-- TODO: Add more testing data
INSERT INTO towers
    (id)
VALUES
    ('03ef3b8aaa7304f741880550a1c0ee9243d62569ffc168926b5989b17c8234149f'),
    ('03ef3b8aaa7304f741880550a1c0ee9243d62569ffc168926b5989b17c8234149g');
