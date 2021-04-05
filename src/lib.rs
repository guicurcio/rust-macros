// This is valid syntax going in; a valid rust program going out.
macro_rules! dec {
    ($x:ident) => {
        $x += 1;
    };
}

#[test]
fn foo() {
    let mut x = 42;
    dec!(x);
    assert_eq!(x, 43);
}
