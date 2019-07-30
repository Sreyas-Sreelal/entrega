CREATE TABLE `order`(
    order_id VARCHAR(40) PRIMARY KEY,
    product_id VARCHAR(40),
    user_id INTEGER NOT NULL,
    ordered_date INTEGER NOT NULL,
    expected_date INTEGER NOT NULL,
    order_status VARCHAR(10) DEFAULT 'pending',
    FOREIGN KEY(product_id) REFERENCES product(product_id),
    FOREIGN KEY(user_id) REFERENCES user(user_id)
)