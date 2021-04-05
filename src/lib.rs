// This is valid syntax going in; a valid rust program going out.
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
}

#[test]
fn empty_vec() {
    let mut x: Vec<u32> = avec![];
    assert!(x.is_empty());
}
