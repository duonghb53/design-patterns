use crate::{builder::Builder, car::{Car, GPS}};

#[derive(Default, Debug)]
pub struct CarBuilder {
    car: Car
}

impl Builder for CarBuilder {
    fn reset(&mut self) {
        self.car = Car::default();
    }

    fn set_seats(&mut self, seats: u8) {
        self.car.seats = seats;
    }

    fn set_engine(&mut self, engine: crate::car::Engine) {
        self.car.engine = engine;
    }

    fn set_trip_computer(&mut self) {
        
    }

    fn set_gps(&mut self, gps: GPS) {
        self.car.gps = gps;
    }
}

impl CarBuilder {
    pub fn get_result(&self) -> Car {
        self.car.clone()
    }
}
