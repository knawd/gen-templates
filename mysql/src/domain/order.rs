use crate::domain::ShippingAddress;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub amount: f32,
    pub shipping: f32,
    pub tax: f32,
    pub shipping_address: ShippingAddress,
}

impl Order {
    pub fn new(
        order_id: i32,
        product_id: i32,
        quantity: i32,
        amount: f32,
        shipping: f32,
        tax: f32,
        shipping_address: ShippingAddress,
    ) -> Self {
        Self {
            order_id,
            product_id,
            quantity,
            amount,
            shipping,
            tax,
            shipping_address,
        }
    }
}
