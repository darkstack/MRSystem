

pub trait BusSpace {
    // Minimal definition
    fn bus_in(&mut self, addr: u8) -> u8;
    fn bus_out(&mut self, addr: u8, v: u8);
}

