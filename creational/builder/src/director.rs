use crate::{
    builder::Builder,
    car::{Engine, GPS},
};
pub struct Director;

impl Director {
    pub fn make_suv(builder: &mut impl Builder) {
        builder.set_seats(4);
        builder.set_engine(Engine::RegularEngine);
        builder.set_gps(GPS::default());
        builder.set_trip_computer();
    }

    pub fn make_sport_car(builder: &mut impl Builder) {
        builder.set_seats(2);
        builder.set_engine(Engine::SportEngine);
        builder.set_gps(GPS::default());
        builder.set_trip_computer();
    }
}
