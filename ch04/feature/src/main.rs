fn main() {
    feature1();
    if cfg!(feature = "feature1") {
        println!("Feature 1 is enabled!");
    } else {
        println!("Feature 1 is disabled!");
    }

    #[cfg(feature = "feature1")]
    {
        println!("Feature 1 is enabled!");
    }

    #[cfg(not(feature = "feature1"))]
    {
        println!("Feature 1 is disabled!");
    }
}

#[cfg(feature = "feature1")]
fn feature1() {
    println!("Feature 1 is enabled!");
}

#[cfg(not(feature = "feature1"))]
fn feature1() {
    println!("Feature 1 is disabled!");
}
