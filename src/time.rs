#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Time {
    int_time: usize
}

impl Time {
    pub fn init(int_time: usize) -> Self {
        Self {
            int_time
        }
    }

    pub fn float_time(&self) -> f64 {
        self.int_time as f64 / 10.0
    }
}