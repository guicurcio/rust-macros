// This is valid syntax going in; a valid rust program going out.
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)+
        vs
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        for _ in 0..$count {
         vs.push($element)
        }
        vs
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}
#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn try_count() {
    let x: Vec<u32> = avec![42;2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

// Trailing commas. (comma separated was.)
#[test]
fn trailing() {
    let _: Vec<u32> = avec![1, 2, 4, 5, 6, 7, 8, 9,];
}

trait MaxValue {
    fn max_value() -> Self;
}

#[macro_export]
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(u32);
max_impl!(i32);
max_impl!(u64);
