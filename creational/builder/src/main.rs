use car_builder::CarBuilder;

use crate::{builder::Builder, car::GPS, director::Director};

mod builder;
mod car;
mod car_builder;
mod director;

fn main() {
    let mut car_builder = CarBuilder::default();
    Director::make_suv(&mut car_builder);
    println!("{:?}", car_builder);
    Director::make_sport_car(&mut car_builder);
    println!("{:?}", car_builder);
    car_builder.set_seats(4);
    car_builder.set_engine(car::Engine::ClassEngine);
    car_builder.set_gps(GPS {
        latitude: 100.0,
        longitude: 100.0,
    });
    println!("{:?}", car_builder);
    let car = car_builder.get_result();
    println!("{:?}", car);
}
