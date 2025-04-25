# Setup

```
create database col_test;
\c col_test;
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);
INSERT INTO users (name)
VALUES
    ('Jane Smith'),
    ('Alex Johnson'),
    ('Sarah Williams');
```
