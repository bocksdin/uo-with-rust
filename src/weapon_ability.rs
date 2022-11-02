#[repr(C)]
pub struct WeaponAbility {
  pub base_mana: i32,
  pub accuracy_bonus: i32,
  pub damage_scalar: f64,
  pub consume_ammo: bool,
}

impl Default for WeaponAbility {
  fn default() -> WeaponAbility {
    WeaponAbility {
      base_mana: 0,
      accuracy_bonus: 0,
      damage_scalar: 1.,
      consume_ammo: true,
    }
  }
}

impl WeaponAbility {
  fn new() -> WeaponAbility {
    WeaponAbility { ..Default::default() }
  }
}

#[no_mangle]
pub extern fn new_weapon_ability() -> WeaponAbility {
  WeaponAbility::new()
}