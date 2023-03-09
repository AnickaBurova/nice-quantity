
pub trait Value: Sized + Copy + std::fmt::Display {
    fn more(self, other: Self) -> bool;
    fn divide(self, other: Self) -> Self;
    fn minus(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn is_one(self) -> bool;
}

impl Value for i64 {
    fn more(self, other: Self) -> bool {
        self > other
    }

    fn divide(self, other: Self) -> Self {
        self / other
    }

    fn minus(self, other: Self) -> Self {
        self - other
    }

    fn mul(self, other: Self) -> Self {
        self * other
    }

    fn is_one(self) -> bool {
        self == 1
    }
}

impl Value for u128 {
    fn more(self, other: Self) -> bool {
        self > other
    }

    fn divide(self, other: Self) -> Self {
        self / other
    }

    fn minus(self, other: Self) -> Self {
        self - other
    }

    fn mul(self, other: Self) -> Self {
        self * other
    }

    fn is_one(self) -> bool {
        self == 1
    }
}

pub trait StringAppend<V: Value> {
    fn append_value_s(&mut self, value: V, units: &str);
    fn append_value(&mut self, value: V, units: &str);
    fn append_divider(&mut self);
}

#[cfg(feature = "string-builder")]
impl<V: Value> StringAppend<V> for string_builder::Builder {
    fn append_value_s(&mut self, value: V, units: &str) {
        self.append(format!("{}{}s", value, units));
    }
    fn append_value(&mut self,value: V, units: &str) {
        self.append(format!("{}{}", value, units));
    }


    fn append_divider(&mut self) {
        if self.len() > 0 {
            self.append(" ");
        }
    }
}

impl<V: Value> StringAppend<V> for String {
    fn append_value_s(&mut self, value: V, units: &str) {
        self.push_str(&format!("{}{}s", value, units));
    }
    fn append_value(&mut self, value: V, units: &str) {
        self.push_str(&format!("{}{}", value, units));
    }

    fn append_divider(&mut self) {
        if !self.is_empty() {
            self.push_str(" ");
        }
    }
}


/// Format value using provided units and their total values.
pub fn custom_format<'b, 'a: 'b, B: StringAppend<V>, V: Value + 'b, I: Iterator<Item = &'b (V, &'a str, &'a str)>>(builder: &mut B, units: I, short_units: bool, mut value: V) {
    for (total, long, short) in units {
        // println!("value = {}, total = {}, unit = {}", value, total, long);
        if value.more(*total) {
            let val = value.divide(*total);
            value = value.minus(val.mul(*total));
            // println!("val = {}", val);
            let unit = if short_units { short } else { long };
            builder.append_divider();
            if !val.is_one() && !short_units {
                builder.append_value_s(val, unit);
            } else {
                builder.append_value(val, unit);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_custom_format() {

    }
}