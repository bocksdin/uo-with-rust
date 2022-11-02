use super::IEntity;

pub struct IDamageable {
    pub serial: Serial,
    pub location: Point3D,
    pub map: Map,
    pub no_move_hs: bool,
    pub direction: Direction,
    pub name: String,
    pub hue: i32,
    pub deleted: bool,
    pub hits: i32,
    pub hits_max: i32,
    pub alive: bool,
    pub physical_resistance: i32,
    pub fire_resistance: i32,
    pub cold_resistance: i32,
    pub poison_resistance: i32,
    pub energy_resistance: i32,
}

impl IDamageable {
    pub fn delete() {}

    pub fn process_delta() {}

    pub fn invalidate_properties() {}

    pub fn on_stats_query(m: &Mobile) {}

    pub fn damage(amount: i32, attacker: &Mobile) -> i32 {}

    pub fn play_sound(sound_id: i32) {}

    pub fn moving_effect(
        to: &IEntity,
        item_id: i32,
        speed: i32,
        duration: i32,
        fixed_direction: bool,
        explodes: bool,
        hue: Option<i32>,
        render_mode: Option<i32>,
    ) {
    }

    pub fn moving_particles(
        to: &IEntity,
        item_id: i32,
        speed: i32,
        duration: i32,
        fixed_direction: bool,
        explodes: bool,
        effect: i32,
        explode_effect: i32,
        explode_sound: i32,
        hue: Option<i32>,
        layer: Option<&EffectLayer>,
        unknown: Option<i32>,
    ) {
    }

    pub fn fixed_effect(
        item_id: i32,
        speed: i32,
        duration: i32,
        hue: Option<i32>,
        render_mode: Option<i32>,
    ) {
    }

    pub fn fixed_particles(
        item_id: i32,
        speed: i32,
        duration: i32,
        effect: i32,
        layer: &EffectLayer,
        hue: Option<i32>,
        render_mode: Option<i32>,
        unknown: Option<i32>,
    ) {
    }

    pub fn bolt_effect(hue: i32) {}
}
