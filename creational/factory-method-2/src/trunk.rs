use crate::transport::*;

pub struct Trunk;

impl Transport for Trunk {
    fn deliver(&self) {
        println!("Trunk Transport!");
    }
}

pub struct RoadLogistics;

impl TransportDeliver for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Trunk)
    }
}
