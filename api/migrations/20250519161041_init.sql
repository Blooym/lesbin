CREATE TABLE pastes (
    id TEXT NOT NULL PRIMARY KEY,
    encryptedTitle TEXT NOT NULL,
    encryptedContent TEXT NOT NULL,
    syntaxType TEXT NOT NULL DEFAULT 'plaintext',
    deletionKey TEXT NOT NULL,
    expiresAt INTEGER,
    createdAt INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE TABLE paste_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pasteId TEXT NOT NULL REFERENCES pastes(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    decryptionKey TEXT NOT NULL,
    createdAt INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);