pub mod time;
pub mod distance;
pub mod macros;

use std::marker::PhantomData;
use std::ops::Deref;
use num_traits::Float;

/// Measure base struct.
///
/// This struct is basically for storing unit value. The unit type is stored
/// in generic and wrapped by a [PhantomData].
///
/// The generic T refers to the value type. It has to be a Float, and can be
/// converted from f64 (in order to be divided by a constant).
///
/// Examples:
/// ```rust
/// use unit_rs::{Measure, measure};
/// use unit_rs::time::{Minute, Second};
///
/// let minute = Measure::<_, Minute>::new(1.);
/// let second: Measure<_, Second> = minute.into();
/// // Note: Deref has been implemented for Measure, so you can use
/// // *second to get unit value.
/// assert_eq!(*second, 60.);
///
/// // or use the macro...
///
/// let minute = measure!(Minute of 1.);
/// let second: Measure<_, Second> = minute.into();
/// assert_eq!(*second, 60.);
/// ```
#[derive(Copy, Clone)]
pub struct Measure<T: Float + From<f32>, U: Unit> {
    pub value: T,
    pub unit: PhantomData<U>
}

impl<T: Float + From<f32>, U: Unit> Measure<T, U> {
    pub fn new(value: T) -> Measure<T, U> {
        Measure {
            value,
            unit: PhantomData
        }
    }
}

impl<T: Float + From<f32>, U: Unit> Deref for Measure<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub enum UnitType {
    Time, Distance
}

pub trait Unit {
    const TYPE: UnitType;
}