CREATE TABLE user(
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name VARCHAR(45) UNIQUE NOT NULL,
    password VARCHAR(256) NOT NULL,
    email VARCHAR(254) UNIQUE NOT NULL,
    display_name VARCHAR(45) NOT NULL,
    address TEXT NOT NULL
)