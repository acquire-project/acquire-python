macro_rules! cvt {
    ($TA:ty => $TB:ty, $($A:ident => $B:ident),+) => {
        crate::components::macros::cvt!(@tryfrom $TA, $TB,  $($A => $B),+);
        crate::components::macros::cvt!(@into $TA, $TB,  $($A => $B),+);
    };
    (@tryfrom $TA:ty, $TB:ty, $($A:ident => $B:ident),+) => {
        impl TryFrom<$TB> for $TA {
            type Error=anyhow::Error;
        
            fn try_from(value: $TB) -> Result<Self, Self::Error> {
                match value as $TB {
                    $(
                        core_runtime::$B => Ok(<$TA>::$A),
                    )+
                    _ => Err(anyhow!("Unknown {}: {}",stringify!($TA),value))
                }
            }
        }
    };
    (@into $TA:ty, $TB:ty, $($A:ident => $B:ident),+) => {
        impl Into<$TB> for $TA {
            fn into(self) -> $TB {
                match self {
                    $(
                        <$TA>::$A => core_runtime::$B,
                    )+
                } 
            }
        }
    }
}
pub(crate) use cvt;