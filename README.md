Format a quantity into a string

There is a custom formatting in the `fmt` module.
Can use custom string builder, at the moment using "string-builder" crate, which can be disabled removing the default feature "string-builder".

The duration (for chrono enable feature "chrono") has modules `to_milliseconds` and `to_microseconds` (must be enabled in features "micro").
Float can be interpreted as seconds using `seconds` module.
```rust
use nice_quantity::seconds::AsSeconds;
use chrono::Duration;
use nice_quantity::to_milliseconds::{ToMilliseconds, Precision};

fn main() {
    let dur = 55.3.as_seconds();
    println!("Duration = {}", dur.nice_duration(Default::default(), true));
    // "55s 3ms"
    let d = Duration::milliseconds(63256);
    println!("{}", d.nice_duration(Precision::Seconds, true));
    // 1m 3s
    println!("{}", d.nice_duration(Precision::Minutes, true));
    // 1m
}
```