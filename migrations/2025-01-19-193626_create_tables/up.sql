-- Your SQL goes here
CREATE TABLE guests (
    name TEXT NOT NULL PRIMARY KEY,
    email TEXT,
    attending BOOLEAN NOT NULL DEFAULT 0,
    meal_preference TEXT,
    meal_extra_info TEXT
);

CREATE TABLE invitations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    lead_guest_name TEXT NOT NULL,
    sent BOOLEAN NOT NULL DEFAULT 0,
    acknowledged BOOLEAN NOT NULL DEFAULT 0,
    code TEXT NOT NULL,
    FOREIGN KEY (lead_guest_name) REFERENCES guests (name)
);