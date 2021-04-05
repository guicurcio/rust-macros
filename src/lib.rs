// This is valid syntax going in; a valid rust program going out.
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        // creating an array of expressions.
        let count = [$($element),*].len();
        let mut vs = Vec::with_capacity(count);
        $(vs.push($element);)+
        vs
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        // vs.extend(std::iter::repeat($element).take(count));
        vs.resize($count,$element);
        vs
    }};
}

// star would mean 0 or more repetitions; rather than plus which is one or more.
// A bunch of reallocations of a vector if we create a new one and then we push.
// Reallocating all the elements when we push and increasing the elements in the vector.
// We could allocate by the number when we know the count.
// Vec::with_capacity($count);
// ...
// vs.extend(std::iter::repeat($element).take(count));
// or
// vs.resize(count,$element);
// We need to get the count for the naive implementation without count; that is, without a second argument.

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

#[test]
fn try_count_nonliteral() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap();2];
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

// adding compile fail as test on doc-tests

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "Supposed-to-fail-no-strings"];
/// ```
pub struct CompileFailTest;
