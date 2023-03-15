pub trait Transport {
    fn deliver(&self);
}

pub trait TransportDeliver {
    fn create_transport(&self) -> Box<dyn Transport>;

    fn deliver(&self) {
        let transport = self.create_transport();
        transport.deliver();
    }
}