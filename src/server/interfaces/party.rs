pub struct IParty;

impl IParty {
    pub fn on_stam_changed(m: &Mobile) {}

    pub fn on_mana_changed(m: &Mobile) {}

    pub fn on_stats_query(beholder: &Mobile, beheld: &Mobile) {}
}
