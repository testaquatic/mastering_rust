macro_rules! conditional_compile {
    () => {
        #[cfg(feature = "feature_flag")]
        fn conditional_function_enabled() {
            println!("Feature flag is enabled!");
        }

        #[cfg(not(feature = "feature_flag"))]
        fn conditional_function_disabled() {
            println!("Feature flag is disabled!");
        }
    };
}

fn main() {
    conditional_compile!();

    #[cfg(feature = "feature_flag")]
    conditional_function_enabled();

    #[cfg(not(feature = "feature_flag"))]
    conditional_function_disabled();
}
