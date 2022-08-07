#[macro_export]
macro_rules! impl_new
{
    ($to:ty,$($v:ident: $t:ty),*)  => {

        impl $to {
            pub fn new($($v: $t),*) -> $to
            {
                Self {
                    $($v),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_default {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self { Self::new() }
        }
    };
}
