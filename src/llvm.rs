use paste::*;

//https://llvm.org/docs/LangRef.html#llvm-clmul-intrinsic
//This is an overloaded intrinsic. You can use llvm.clmul on any integer or vectors of integer elements.
//
//
macro_rules! make_llvm_clmul {
    ($(($t:ty,$i:ident)),*) => {
        paste! {
            $(
    #[link_name = "llvm.clmul." $i ""]
    unsafe fn [<ll_clmul_ $t>](a: $t, b: $t) -> $t;
            )*
        }
    };
}

#[allow(unused)]
unsafe extern "C" {

    make_llvm_clmul!((u8, i8), (u16, i16), (u32, i32), (u64, i64), (u128, i128));
}

macro_rules! make_clmul_fn {
    ($($t:ty),*) => {
        paste! {
            $(
                pub fn [<clmul_ $t>](a: $t,b: $t)-> $t {
                    unsafe {
                        [<ll_clmul_ $t>](a,b)
                    }
                }
            )*
        }
    };
}

make_clmul_fn!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u16() {
        let a = 255u16;
        let b = a;

        let c = clmul_u16(a, b);
        assert_eq!(c, 0b101010101010101_u16);
    }

    #[test]
    fn test_u32() {
        let a = 255u32;
        let b = a;

        let c = clmul_u32(a, b);
        assert_eq!(c, 0b101010101010101_u32);
    }

    #[test]
    fn test_u64() {
        let a = 255u64;
        let b = a;

        let c = clmul_u64(a, b);
        assert_eq!(c, 0b101010101010101_u64);
    }

    #[test]
    fn test_u128() {
        let a = 255u128;
        let b = a;

        let c = clmul_u128(a, b);
        assert_eq!(c, 0b101010101010101_u128);
    }
}
