use crate::car::{Engine, GPS};

pub trait Builder {
    fn reset(&mut self);
    fn set_seats(&mut self, seats: u8);
    fn set_engine(&mut self, engine: Engine);
    fn set_trip_computer(&mut self);
    fn set_gps(&mut self, gps: GPS);
}
