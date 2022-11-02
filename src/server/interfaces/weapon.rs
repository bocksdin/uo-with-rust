use super::IDamageable;
use std::time::Duration;

pub struct IWeapon {
    pub max_range: i32,
}

impl IWeapon {
    pub fn on_before_swing(attacker: &Mobile, damageable: &IDamageable) {}

    pub fn on_swing(attacker: &Mobile, damageable: &IDamageable) -> Duration {}

    // min max are 'out'
    pub fn get_status_damage(from: &Mobile, min: i32, max: i32) {}

    pub fn get_delay(attacker: Mobile) -> Duration {}
}
