CREATE TABLE product(
    product_id VARCHAR(40) PRIMARY KEY,
    product_name VARCHAR(120) NOT NULL,
    description TEXT,
    price FLOAT NOT NULL,
    rating FLOAT DEFAULT 0.0    
)