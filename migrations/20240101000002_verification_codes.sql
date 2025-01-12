CREATE TABLE IF NOT EXISTS verification_codes (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL,
    code TEXT NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_verification_codes_email ON verification_codes(email); 