use std::time::Duration;

pub trait DurationAsNano {
    fn as_nanos(&self) -> u64;
}

impl DurationAsNano for Duration {
    fn as_nanos(&self) -> u64 {
        let mut nanos: u64 = self.subsec_nanos().into();
        nanos += self.as_secs() * 1_000_000_000;

        return nanos;
    }
}
