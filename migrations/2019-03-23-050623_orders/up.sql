CREATE TABLE `order`(
    order_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    product_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    ordered_date INTEGER NOT NULL,
    expected_date INTEGER NOT NULL,
    FOREIGN KEY(product_id) REFERENCES product(product_id),
    FOREIGN KEY(user_id) REFERENCES user(user_id)
)