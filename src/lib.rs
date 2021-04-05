// This is valid syntax going in; a valid rust program going out.
macro_rules! dec {
    ($arg1:ty => $arg2:ident) => {
        type $arg2 = $arg1;
    };
}

dec!(
    u32 => alsou32
);
