CREATE TABLE admins(
    user_id INTEGER PRIMARY KEY,
    FOREIGN KEY(user_id) REFERENCES users(user_id)
)