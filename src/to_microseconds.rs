//! Get the microseconds value out of some value representing duration.
//! The `nice_duration` function will format the duration in to a string, where it will skip
//! zeros and will start with the highest possible unit.
//! Precision can be used to set choose the lowest possible unit to format into.
//! `short_units` switches between formatting full unit names or their short representation.
use crate::fmt::custom_format;

#[repr(usize)]
pub enum PrecisionMicro {
    Microseconds = 0,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}
impl Default for PrecisionMicro {
    fn default() -> Self {
        PrecisionMicro::Microseconds
    }
}
/// Get the microseconds from the value
pub trait ToMicroseconds {
    fn to_microseconds(&self) -> u128;
    fn nice_duration(
        &self,
        precision: PrecisionMicro,
        short_units: bool,
    ) -> String {
        let value = self.to_microseconds();
        if value == 0 {
            return "0s".to_string();
        }
        lazy_static::lazy_static! {
        static ref CHUNKS: Vec<(u128, &'static str, &'static str)> = {
            [
                (1, "microsecond", "µs"),
                (1000, "millisecond", "ms"),
                (1000, "second", "s"),
                (60, "minute", "m"),
                (60, "hour", "h"),
                (24, "day", "d"),
                (7, "week", "w"),
            ].into_iter().scan(1, |acc, (mul, long, short)| {
                *acc = *acc * mul;
                Some((*acc, long, short ))
            }).collect::<Vec<_>>()
                // (604800000, "week"), (86400000, "day"), (3600000, "hour"), (60000, "minute"), (10000, "second")]
                // .map(|(value, long)| {
                //     let short = long[0];
                //     (value, value / 1000, long, short)
                // })
            };
        }
        #[cfg(feature = "string-builder")]
        let mut result = string_builder::Builder::new(32);
        #[cfg(not(feature = "string-builder"))]
        let mut result = String::new();
        custom_format(&mut result,
        CHUNKS[precision as usize..]
            .iter()
            .rev(),
            short_units,
            value);

        #[cfg(feature = "string-builder")]
        let result = result.string().expect("Failed to generate string");

        result
    }

}

impl ToMicroseconds for std::time::Duration {
    fn to_microseconds(&self) -> u128 {
        self.as_micros()
    }
}
impl ToMicroseconds for &std::time::Duration {
    fn to_microseconds(&self) -> u128 {
        self.as_micros()
    }
}

#[cfg(feature = "chrono")]
impl ToMicroseconds for chrono::Duration {
    fn to_microseconds(&self) -> u128 {
        self.num_microseconds().unwrap() as u128
    }
}


#[cfg(feature = "chrono")]
impl ToMicroseconds for &chrono::Duration {
    fn to_microseconds(&self) -> u128 {
        self.num_microseconds().unwrap() as u128
    }
}
