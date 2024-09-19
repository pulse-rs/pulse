#[macro_export]
macro_rules! include_files {
    ( $( $file:expr ),* ) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push(include_str!($file).to_string());
            )*
            vec
        }
    };
}
