/// Generate a measure
///
/// Examples:
/// ```rust
/// use unit_rs::{measure, Measure};
/// use unit_rs::distance::Meter;
///
/// let meters = measure!(Meter of 3.);
/// ```
#[macro_export]
macro_rules! measure {
    ($unit:tt of $value:expr) => {
        Measure::<_, $unit>::new($value)
    };
}

/// Convert a [crate::Measure] into a desired unit
///
/// Examples:
/// ```rust
/// use unit_rs::{convert, measure, Measure};
/// use unit_rs::distance::{Meter, Kilometer};
///
/// let meters = measure!(Meter of 3.);
/// let kilometers = convert!(Kilometer from meters);
/// ```
#[macro_export]
macro_rules! convert {
    ($left:tt from $right:expr) => {
        Measure::<_, $left>::from($right)
    };
}

#[macro_export]
macro_rules! define_convert {
    ($origin:tt to $after:tt, |$x:ident| $calc:expr) => {
        impl<T> From<Measure<T, $origin>> for Measure<T, $after>
        where
            T: Float + From<f32>
        {
            fn from(value: Measure<T, $origin>) -> Self {
                let mut $x = *value;
                $x = $calc;
                Measure::<T, $after>::new($x)
            }
        }
    }
}


#[cfg(test)]
mod macros_test {
    use std::any::{type_name, type_name_of_val};
    use std::marker::PhantomData;

    use crate::distance::Meter;
    use crate::Measure;

    #[test]
    fn measure_macros() {
        let meters = measure!(Meter of 3.0);
        assert_eq!(type_name_of_val(&meters.unit), type_name::<PhantomData<Meter>>());
        assert_eq!(*meters, 3.0);
    }
}