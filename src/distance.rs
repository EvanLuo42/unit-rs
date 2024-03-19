use num_traits::Float;
use crate::{define_convert, Measure, Unit, UnitType};

#[derive(Eq, PartialEq, Debug)]
pub struct Meter;

impl Unit for Meter {
    const TYPE: UnitType = UnitType::Distance;
}

pub struct Kilometer;

impl Unit for Kilometer {
    const TYPE: UnitType = UnitType::Distance;
}

define_convert!(Meter to Kilometer, |origin| origin / 1000.0.into());
define_convert!(Kilometer to Meter, |origin| origin * 1000.0.into());

#[cfg(test)]
mod distance_test {
    use crate::distance::{Kilometer, Meter};
    use crate::{convert, Measure, measure};

    #[test]
    fn meters_to_kilometers() {
        let meters = measure!(Meter of 1000.);
        let kilometers = convert!(Kilometer from meters);
        assert_eq!(*kilometers, 1.);
    }

    #[test]
    fn kilometers_to_meters() {
        let kilometers = measure!(Kilometer of 1.);
        let meters = convert!(Meter from kilometers);
        assert_eq!(*meters, 1000.);
    }
}
