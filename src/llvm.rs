use paste::*;

//https://llvm.org/docs/LangRef.html#llvm-clmul-intrinsic
//This is an overloaded intrinsic. You can use llvm.clmul on any integer or vectors of integer elements.
//
//
macro_rules! make_llvm_clmul {
    ($($t:ty),*) => {
        paste! {
            $(
    #[link_name = "llvm.clmul." $t ""]
    unsafe fn [<ll_clmul_ $t>](a: $t, b: $t) -> $t;
            )*
        }
    };
}


#[allow(unused)]
unsafe extern "C" {

make_llvm_clmul!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,usize,isize);
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

make_clmul_fn!(u8,i8,u16,i16,u32,i32,u64,i64,u128,i128);


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i16() {
        let a=255i16;
        let b=a;

        let c=clmul_i16(a,b);
        assert_eq!(c,0b101010101010101_i16);
    }

    #[test]
    fn test_u32() {
        let a=255u32;
        let b=a;

        let c=clmul_u32(a,b);
        assert_eq!(c,0b101010101010101_u32);
    }
}
