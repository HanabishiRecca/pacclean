use std::fmt::{Display, Formatter, Result};

const UNITS: &[&str] = &["KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
const FACTOR: u64 = 1024;
const FACTOR_F: f64 = FACTOR as f64;

pub struct ByteFormat(pub u64);

impl Display for ByteFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.0 < FACTOR {
            return write!(f, "{} B", self.0);
        }

        let mut result = (self.0 as f64) / FACTOR_F;
        let mut n = 0;

        while result >= FACTOR_F {
            result /= FACTOR_F;
            n += 1;
        }

        write!(
            f,
            "{} {}",
            (result * 100.0).round() / 100.0,
            UNITS.get(n).copied().unwrap_or("???")
        )
    }
}
