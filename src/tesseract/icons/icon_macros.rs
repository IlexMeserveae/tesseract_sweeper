#[macro_export]
macro_rules! load_icons {
    ($($style:expr),* ; $($icon:expr),*) => {
        product_helper_1!([$([$icon])*][$([$style])*])
    }
}
macro_rules! product_helper_1 {
    ([$([$styles:expr])*]$icon:tt) => {
        product_helper_2!($([[$styles]$icon])*)
    }
}
macro_rules! product_helper_2 {
    ($([[$style:expr][$([$icon:expr])*]])*) => {
        [$((stringify!($style), [$((stringify!($icon), include_image!(concat!("assets/",
        stringify!($icon), "_", stringify!($style), ".png")))),*])),*]
    }
}

// Above From: https://users.rust-lang.org/t/macro-rules-any-way-to-perform-cartesian-product-without-recursion/49897/3
