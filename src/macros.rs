#[macro_export]
macro_rules! print_opt {
    ($label:literal, $opt:expr) => {
        if let Some(v) = &$opt {
            println!("{:<12}: {}", $label, v);
        }
    };
}
