use util::*;

pub type IntType = u64;
pub struct Race {
    time: IntType,
    distance: IntType,
}

impl Race {
    pub fn new(time: IntType, distance: IntType) -> Self {
        Self { time, distance }
    }

    pub fn curve_above_distance(&self) -> Range<IntType> {
        // x = (t +/- sqrt(t.pow(2) - 4 * d)) / 2
        // Subtract negligent amount from radius to allow
        // ceil function to work for whole numbers
        let radius = IntType::sqrt(self.time.pow(2) - 4 * self.distance) / 2.0 - 0.00000001;
        let center = self.time as f64 / 2.0;
        IntType::ceil(center - radius)..IntType::ceil(center + radius)
    }
}
