use ship::SeaLogistics;
use std::fmt::Error;
use std::io;
use std::num::ParseIntError;
use transport::TransportDeliver;
use trunk::RoadLogistics;

mod ship;
mod transport;
mod trunk;

fn main() {
    
    loop {
        println!("\n");
        println!("Enter choose your Transport!");
        println!("1. Trunk");
        println!("2. Ship");
        println!("Other. Exit");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let type_transport = input.trim().parse::<i32>();
                let result = match type_transport {
                    Ok(i) => initialize(i),
                    Err(_) => None,
                };

                if result.is_some() {
                    result.unwrap().deliver();
                } else {
                    break;
                }
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
}

fn initialize(type_transport: i32) -> Option<&'static dyn TransportDeliver> {
    if type_transport == 1 {
        Some(&RoadLogistics)
    } else if type_transport == 2 {
        Some(&SeaLogistics)
    } else {
        None
    }
}
