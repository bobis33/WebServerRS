CREATE SCHEMA IF NOT EXISTS public;

CREATE TABLE IF NOT EXISTS "User" (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255),
    name VARCHAR(255),
    role VARCHAR(255) NOT NULL DEFAULT 'USER',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

INSERT INTO "User" (name, email, password, role)
VALUES
    ('Alice', 'alice@example.com', '$2b$10$1HR4vKXvBXlXwA5DmRIFg.lkppvyw6yOxMcay15t7.N84ePXgjUE2', 'USER'),
    ('Bob', 'bob@example.com', '$2b$10$1HR4vKXvBXlXwA5DmRIFg.lkppvyw6yOxMcay15t7.N84ePXgjUE2', 'USER'),
    ('Charlie', 'charlie@example.com', '$2b$10$1HR4vKXvBXlXwA5DmRIFg.lkppvyw6yOxMcay15t7.N84ePXgjUE2', 'USER'),
    ('David', 'david@example.com', '$2b$10$1HR4vKXvBXlXwA5DmRIFg.lkppvyw6yOxMcay15t7.N84ePXgjUE2', 'USER'),
    ('Eve', 'eve@example.com', '$2b$10$1HR4vKXvBXlXwA5DmRIFg.lkppvyw6yOxMcay15t7.N84ePXgjUE2', 'USER'),
    ('admin', 'admin@admin.com', '$2b$10$vXissZGi9Uuel3dHKLTDUe5X9RrFsATPHzsnEZzn5qgkRARblAeSa', 'ADMIN');
