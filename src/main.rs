use std::marker::PhantomData;
use num_traits::Num;

fn main() {
    let a = Measure::new::<Second>(1.);
}

struct Measure<T: Num + From<f64>, U: Unit> {
    value: T,
    unit: PhantomData<U>
}

impl<T: Num + From<f64>, U: Unit> Measure<T, U> {
    fn new<V: Unit>(value: T) -> Measure<T, V> {
        Measure {
            value,
            unit: PhantomData
        }
    }
}

impl<T: Num + From<f64>> From<Measure<T, Second>> for Measure<T, Minute> {
    fn from(value: Measure<T, Second>) -> Self {
        Measure::<T, Minute>::new::<Minute>(value.value / <T as From<f64>>::from(60.))
    }
}

enum UnitType {
    Time
}

trait Unit {
    const TYPE: UnitType;
}

struct Second;

impl Unit for Second {
    const TYPE: UnitType = UnitType::Time;
}

struct Minute;

impl Unit for Minute {
    const TYPE: UnitType = UnitType::Time;
}