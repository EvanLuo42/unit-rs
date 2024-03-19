use num_traits::Float;
use crate::{define_convert, Measure, Unit, UnitType};

pub struct Second;

impl Unit for Second {
    const TYPE: UnitType = UnitType::Time;
}

pub struct Minute;

impl Unit for Minute {
    const TYPE: UnitType = UnitType::Time;
}

define_convert!(Second to Minute, |origin| origin / 60.0.into());
define_convert!(Minute to Second, |origin| origin * 60.0.into());

#[cfg(test)]
mod time_test {
    use crate::{convert, Measure, measure};
    use crate::time::{Minute, Second};

    #[test]
    fn seconds_to_minutes() {
        let seconds = measure!(Second of 60.);
        let minutes = convert!(Minute from seconds);
        assert_eq!(*minutes, 1.)
    }

    #[test]
    fn minutes_to_seconds() {
        let minutes = measure!(Minute of 1.);
        let seconds = convert!(Second from minutes);
        assert_eq!(*seconds, 60.)
    }
}