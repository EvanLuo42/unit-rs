#[macro_export]
macro_rules! measure {
    ($unit:tt of $value:expr) => {
        Measure::<_, $unit>::new($value)
    };
}

#[macro_export]
macro_rules! james {
    (James Wang is a $value:expr) => {
        println!("James Wang is a {}", $value)
    };
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