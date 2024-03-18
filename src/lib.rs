pub mod time;
mod distance;

use std::marker::PhantomData;
use std::ops::Deref;
use num_traits::Float;

#[derive(Copy, Clone)]
pub struct Measure<T: Float + From<f32>, U: Unit> {
    value: T,
    unit: PhantomData<U>
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