CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT,
    first_name TEXT NOT NULL,
    last_name TEXT
);

CREATE TABLE spams (
    message_id INTEGER PRIMARY KEY,
    created DATE NOT NULL,
    owner INTEGER NOT NULL,
    FOREIGN KEY(owner) REFERENCES users(id)
);

CREATE TABLE spam_voters (
    user_id INTEGER NOT NULL,
    spam_id INTEGER NOT NULL,
    PRIMARY KEY(user_id, spam_id),
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(spam_id) REFERENCES spams(id)
);