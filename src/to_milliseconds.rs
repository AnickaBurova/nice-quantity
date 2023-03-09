use crate::fmt::custom_format;

/// Get the milliseconds from the value
pub trait ToMilliseconds {
    /// Get the milliseconds from the value
    fn to_milliseconds(&self) -> i64;
    /// Format the value into a nice duration string
    fn nice_duration(&self, precision: Precision, short_units: bool) -> String {
        let value = self.to_milliseconds();
        if value == 0 {
            return "0s".to_string();
        }
        lazy_static::lazy_static! {
        static ref CHUNKS: Vec<(i64, &'static str, &'static str)> = {
            [
                (1, " millisecond", "ms"),
                (1000, " second", "s"),
                (60, " minute", "m"),
                (60, " hour", "h"),
                (24, " day", "d"),
                (7, " week", "w"),
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

/// Minimum precision to format the duration
#[repr(usize)]
pub enum Precision {
    /// Format the duration in milliseconds
    Milliseconds = 0,
    /// Format the duration in seconds, discarding milliseconds
    Seconds,
    /// Format the duration in minutes, discarding seconds and further
    Minutes,
    /// Format the duration in hours, discarding minutes and further
    Hours,
    /// Format the duration in days, discarding hours and further
    Days,
    /// Format the duration in weeks, discarding days and further
    Weeks,
}

impl Default for Precision {
    fn default() -> Self {
        Precision::Milliseconds
    }
}


#[cfg(feature = "chrono")]
impl ToMilliseconds for chrono::Duration {
    fn to_milliseconds(&self) -> i64 {
        self.num_milliseconds()
    }
}

impl ToMilliseconds for std::time::Duration {
    fn to_milliseconds(&self) -> i64 {
        self.as_millis() as i64
    }
}

#[cfg(feature = "chrono")]
impl ToMilliseconds for &chrono::Duration {
    fn to_milliseconds(&self) -> i64 {
        self.num_milliseconds()
    }
}

impl ToMilliseconds for &std::time::Duration {
    fn to_milliseconds(&self) -> i64 {
        self.as_millis() as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::seconds::AsSeconds;
    use super::*;
    macro_rules! cmp {
        ($left: expr, $right: literal) => {
            assert_eq!($left.nice_duration(Default::default(), true), $right);
        }
    }
    #[test]
    fn test_millis() {
        assert_eq!(
        10.0.as_seconds().nice_duration(Default::default(), true),
            "10s"
            );
        cmp!(63.234.as_seconds(), "1m 3s 234ms");
        cmp!(63.234.as_seconds(), "1m 3s 234ms");
    }
}
