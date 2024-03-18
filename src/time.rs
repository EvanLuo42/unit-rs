use num_traits::Float;
use crate::{Measure, Unit, UnitType};

pub struct Second;

impl Unit for Second {
    const TYPE: UnitType = UnitType::Time;
}

pub struct Minute;

impl Unit for Minute {
    const TYPE: UnitType = UnitType::Time;
}

impl<T: Float + From<f32>> From<Measure<T, Second>> for Measure<T, Minute> {
    fn from(value: Measure<T, Second>) -> Self {
        Measure::<T, Minute>::new(*value / 60.0.into())
    }
}

impl<T: Float + From<f32>> From<Measure<T, Minute>> for Measure<T, Second> {
    fn from(value: Measure<T, Minute>) -> Self {
        Measure::<T, Second>::new(*value * 60.0.into())
    }
}

#[cfg(test)]
mod time_test {
    use crate::{Measure, measure};
    use crate::time::{Minute, Second};

    #[test]
    fn seconds_to_minutes() {
        let second = measure!(Second of 60.);
        let minute: Measure<_, Minute> = second.into();
        assert_eq!(*minute, 1.)
    }

    #[test]
    fn minutes_to_seconds() {
        let minute = measure!(Minute of 1.);
        let second: Measure<_, Second> = minute.into();
        assert_eq!(*second, 60.)
    }
}