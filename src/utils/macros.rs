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
