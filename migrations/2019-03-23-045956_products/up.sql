CREATE TABLE products(
    product_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    product_name VARCHAR(120) NOT NULL,
    price FLOAT NOT NULL,
    rating FLOAT DEFAULT 0.0    
)