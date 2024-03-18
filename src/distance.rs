use num_traits::Float;
use crate::{Measure, Unit, UnitType};

#[derive(Eq, PartialEq, Debug)]
pub struct Meter;

impl Unit for Meter {
    const TYPE: UnitType = UnitType::Distance;
}

pub struct Kilometer;

impl Unit for Kilometer {
    const TYPE: UnitType = UnitType::Distance;
}

impl<T: Float + From<f32>> From<Measure<T, Kilometer>> for Measure<T, Meter> {
    fn from(value: Measure<T, Kilometer>) -> Self {
        Measure::<T, Meter>::new(*value * 1000.0.into())
    }
}

impl<T: Float + From<f32>> From<Measure<T, Meter>> for Measure<T, Kilometer> {
    fn from(value: Measure<T, Meter>) -> Self {
        Measure::<T, Kilometer>::new(*value / 1000.0.into())
    }
}

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
