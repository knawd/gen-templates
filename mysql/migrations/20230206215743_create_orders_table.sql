-- Add migration script here
CREATE TABLE orders (order_id INT, product_id INT, quantity INT, amount FLOAT, shipping FLOAT, tax FLOAT, shipping_address VARCHAR(20));