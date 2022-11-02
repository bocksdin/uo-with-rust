use chrono::{offset::Utc, DateTime};
use std::time::Duration;

pub struct IVendor {
    pub last_restock: DateTime<Utc>,
    pub restock_delay: Duration,
}

impl IVendor {
    pub fn on_buy_items(from: &Mobile, list: &Vec<BuyItemResponse>) -> bool {
        true
    }

    pub fn on_sell_items(from: &Mobile, list: &Vec<SellItemResponse>) -> bool {
        true
    }

    pub fn restock() {}
}
