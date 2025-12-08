CREATE TABLE pastes (
    id TEXT NOT NULL PRIMARY KEY,
    encrypted_title TEXT NOT NULL,
    encrypted_content TEXT NOT NULL,
    encrypted_syntax_type TEXT NOT NULL,
    deletion_key_hash TEXT NOT NULL,
    expires_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);
CREATE INDEX idx_pastes_expires_at ON pastes(expires_at) WHERE expires_at IS NOT NULL;
