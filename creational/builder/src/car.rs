#[derive(Debug, Default, Clone)]
pub enum Engine {
    #[default]
    RegularEngine,
    SportEngine,
    ClassEngine,
}

#[derive(Debug, Default, Clone)]
pub struct GPS {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Car {
    pub seats: u8,
    pub engine: Engine,
    pub gps: GPS,
    pub trip: String,
}
