pub struct IEntity {
    pub serial: Serial,
    pub location: Point3D,
    pub map: Map,
    pub no_move_hs: bool,
    pub direction: Direction,
    pub name: String,
    pub hue: i32,
    pub deleted: bool,
}

impl IEntity {
    pub fn delete() {}

    pub fn process_delta() {}

    pub fn invalidate_properties() {}

    pub fn on_stats_query(m: &Mobile) {}
}
