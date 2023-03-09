use crate::to_milliseconds::ToMilliseconds;
#[cfg(feature = "micro")]
use crate::to_microseconds::ToMicroseconds;


#[cfg(feature = "micro")]
pub trait Available: ToMilliseconds + ToMicroseconds {
}

#[cfg(not(feature = "micro"))]
pub trait Available: ToMilliseconds {
}

pub trait AsSeconds {
    type Seconds: Available;
    fn as_seconds(&self) -> Self::Seconds;
}

pub struct SecondsF64(pub f64);

impl Available for SecondsF64 {}

#[cfg(feature = "micro")]
impl ToMicroseconds for SecondsF64 {
    fn to_microseconds(&self) -> u128 {
        (self.0 * 1000_000.0) as u128
    }
}

impl AsSeconds for f64 {
    type Seconds = SecondsF64;
    fn as_seconds(&self) -> Self::Seconds {
        SecondsF64(*self)
    }
}

impl ToMilliseconds for SecondsF64 {
    fn to_milliseconds(&self) -> i64 {
        (self.0 * 1000.0) as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::seconds::AsSeconds;
    use crate::to_milliseconds::ToMilliseconds;
    #[cfg(feature = "micro")]
    use crate::to_microseconds::ToMicroseconds;

    #[test]
    fn test_as_seconds() {
        let a = 3.2353865.as_seconds();
        assert_eq!(a.to_milliseconds(), 3235);
        #[cfg(feature = "micro")]
        assert_eq!(a.to_microseconds(), 3235386);
    }
}