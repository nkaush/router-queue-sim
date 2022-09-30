use crate::time::Time;

pub struct Packet {
    insert_time: Time
}

impl Packet {
    pub fn new(insert_time: &Time) -> Self {
        Self {
            insert_time: insert_time.clone()
        }
    }

    /// get queueing delay as if packet is sent at time t
    pub fn send(&mut self, t: &Time) -> f64 {
        t.float_time() - self.insert_time.float_time()
    }
}