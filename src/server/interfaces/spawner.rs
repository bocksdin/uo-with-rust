use super::ISpawnable;

pub struct ISpawner {
    pub unlink_on_taming: bool,
    pub home_location: Point3D,
    pub home_range: i32,
}

impl ISpawner {
    pub fn remove(spawn: &ISpawnable) {}

    pub fn get_spawn_properties(spawn: &ISpawnable, list: &ObjectPropertyList) {}

    pub fn get_spawn_context_entries(spawn: &ISpawnable, m: &Mobile, list: &Vec<ContextMenuEntry>) {
    }
}
