macro_rules! dec {
    ($arg1:ty => $arg2:expr; $arg3:path) => {};
}

dec!(
    u32 => x.foo(); std::path
);
