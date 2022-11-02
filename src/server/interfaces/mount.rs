pub struct IMount {
    pub rider: Mobile,
}

impl IMount {
    // amount is 'ref'
    pub fn on_rider_damaged(from: &Mobile, amount: i32, will_kill: bool) {}
}
