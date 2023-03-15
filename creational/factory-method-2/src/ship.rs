use crate::transport::*;

pub struct Ship;

impl Transport for Ship {
    fn deliver(&self) {
        println!("Ship Transport!");
    }
}

pub struct SeaLogistics;

impl TransportDeliver for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship)
    }
}
