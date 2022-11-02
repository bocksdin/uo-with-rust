pub struct ISpell {
    pub id: i32,
    pub is_casting: bool,
}

impl ISpell {
    pub fn on_caster_hurt() {}

    pub fn on_caster_killed() {}

    pub fn on_connection_changed() {}

    pub fn on_caster_moving(d: &Direction) -> bool {
        true
    }

    pub fn check_movement(caster: &Mobile) -> bool {
        true
    }

    pub fn on_caster_equiping(item: &Item) -> bool {
        true
    }

    pub fn on_caster_using_object(o: &Object) -> bool {
        true
    }

    pub fn on_cast_in_town(r: &Region) -> bool {
        true
    }
}
