pub struct ISpawnable {
    pub serial: Serial,
    pub location: Point3D,
    pub map: Map,
    pub no_move_hs: bool,
    pub direction: Direction,
    pub name: String,
    pub hue: i32,
    pub deleted: bool,
}

impl ISpawnable {
    pub fn delete() {}

    pub fn process_delta() {}

    pub fn invalidate_properties() {}

    pub fn on_stats_query(m: &Mobile) {}

    pub fn on_before_spawn(location: &Point3D, map: &Map) {}

    pub fn move_to_world(location: &Point3D, map: &Map) {}

    pub fn on_after_spawn() {}
}
