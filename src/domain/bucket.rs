use std::cmp::min;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Bucket {
    capacity: u16,
    measurement: u16,
}

impl Bucket {
    pub(crate) fn new(capacity: u16) -> Self {
        Bucket {
            capacity,
            measurement: 0,
        }
    }

    pub(crate) fn fill(&mut self) {
        self.measurement = self.capacity;
    }

    pub(crate) fn empty(&mut self) {
        self.measurement = 0;
    }

    pub(crate) fn pour_into(&mut self, other: &mut Bucket) {
        let max_donation = self.measurement;
        let max_receipt = other.capacity - other.measurement;
        let change = min(max_donation, max_receipt);
        self.measurement -= change;
        other.measurement += change;
    }

    pub(crate) fn measurement(&self) -> u16 {
        self.measurement
    }
}
