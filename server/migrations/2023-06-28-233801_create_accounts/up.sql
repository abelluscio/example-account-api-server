-- Table for storing account metadata
CREATE TABLE accounts (
    id UUID DEFAULT(gen_random_uuid()) NOT NULL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    middle_initial CHAR(1),
    created_at TIMESTAMP NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP DEFAULT(null),
    frozen BOOLEAN DEFAULT(false)
);
