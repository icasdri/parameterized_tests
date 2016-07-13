#[macro_export]
macro_rules! parameterized_tests {
    ( $func:path; $( $i:ident: $( $param:expr ),*  => $expected:expr )* ) => {
        $(
            #[test]
            fn $i() {
                $func($( $param ),*, $expected);
            }
        )*
    }
}
