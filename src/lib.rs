pub mod time;

use std::marker::PhantomData;
use num_traits::Num;

pub struct Measure<T: Num + From<f32>, U: Unit> {
    value: T,
    unit: PhantomData<U>
}

impl<T: Num + From<f32>, U: Unit> Measure<T, U> {
    pub fn new(value: T) -> Measure<T, U> {
        Measure {
            value,
            unit: PhantomData
        }
    }
}

pub enum UnitType {
    Time
}

pub trait Unit {
    const TYPE: UnitType;
}