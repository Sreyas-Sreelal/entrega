CREATE TABLE admin(
    user_id INTEGER PRIMARY KEY,
    FOREIGN KEY(user_id) REFERENCES user(user_id)
)