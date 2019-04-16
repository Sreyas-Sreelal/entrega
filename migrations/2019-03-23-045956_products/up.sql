CREATE TABLE product(
    product_id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_name VARCHAR(120) NOT NULL,
    price FLOAT NOT NULL,
    rating FLOAT DEFAULT 0.0    
)