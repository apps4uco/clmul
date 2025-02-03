#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Crate to call the clmul instruction
//!
//! Otherwise known as:
//! * Carry-Less Multiplication
//! * Carry-Less Multiply
//! * Carry-Less Product
//!

mod clmul_demo;
pub mod clmul_inv;
pub mod morton;
mod transpose;

/**
 * clmul
 *
 *
 */
pub fn clmul(a: u64, b: u64) -> u128 {
    //Intel’s PCLMULQDQ instruction (part of the CLMUL extension,

    // Dynamic Detection.
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::is_aarch64_feature_detected;
        if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
            // SAFETY: target_features "neon" and "aes" are available in this block.
            return unsafe { clmul_aarch64_neon(a, b) };
        }
    }
    clmul_nosimd(a, b)
}

/// This intrinsic corresponds to the <c> VPCLMULQDQ </c> instruction.
///
/// \param __X
///    A 128-bit vector of [2 x i64] containing one of the source operands.
/// \param __Y
///    A 128-bit vector of [2 x i64] containing one of the source operands.
/// \param __I
///    An immediate value specifying which 64-bit values to select from the
///    operands. Bit 0 is used to select a value from operand \a __X, and bit
///    4 is used to select a value from operand \a __Y: \n
///    Bit[0]=0 indicates that bits[63:0] of operand \a __X are used. \n
///    Bit[0]=1 indicates that bits[127:64] of operand \a __X are used. \n
///    Bit[4]=0 indicates that bits[63:0] of operand \a __Y are used. \n
///    Bit[4]=1 indicates that bits[127:64] of operand \a __Y are used.
/// \returns The 128-bit integer vector containing the result of the carry-less
///    multiplication of the selected 64-bit values.

// Implementation of clmul using instrinsics (vmull_p64) on Arm Processor with AES
#[cfg(all(
    target_arch = "aarch64",
    target_feature = "neon",
    target_feature = "aes"
))]
unsafe fn clmul_aarch64_neon(a: u64, b: u64) -> u128 {
    // SAFETY: target_features "neon" and "aes" are available in this function.
    core::arch::aarch64::vmull_p64(a, b)
}

// #[cfg(not(any(all(target_arch="aarch64", target_feature="neon"), all(target_arch = "x86_64", target_feature = "pclmulqdq")))]

// Fallback implementation
fn clmul_nosimd(a: u64, b: u64) -> u128 {
    let mut tmp: u128 = b as u128;
    let mut result: u128 = 0;
    for i in 0..64 {
        if a & (1 << i) != 0 {
            result ^= tmp;
        }
        tmp <<= 1;
    }
    result
}

//Code adapted from RISC-V Bitmanip Extension V0.90

// pub fn clmul(rs1: u32, rs2: u32) -> u32
// {
//     let mut x = 0;
//     for i = 0..u32::BITS {
//     if ((rs2 >> i) & 1) != 0 {
//         x ^= rs1 << i;
//     }
//     }
//     x
// }

// uint_xlen_t clmulh(uint_xlen_t rs1, uint_xlen_t rs2)
// {
// uint_xlen_t x = 0;
// for (int i = 1; i < XLEN; i++)
// if ((rs2 >> i) & 1)
// x ^= rs1 >> (XLEN-i);
// return x;
// }
// uint_xlen_t clmulr(uint_xlen_t rs1, uint_xlen_t rs2)
// {
// uint_xlen_t x = 0;
// for (int i = 0; i < XLEN; i++)
// if ((rs2 >> i) & 1)
// x ^= rs1 >> (XLEN-i-1);
// return x;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let result = clmul(2, 2);
        assert_eq!(result, 4);
    }

    fn binary_to_gray(num: u32) -> u32 {
        return num ^ (num >> 1); // The operator >> is shift right. The operator ^ is exclusive or.
    }

    fn gray_to_binary(num: u32) -> u32 {
        todo!()
    }

    fn gray_to_binary32_no_intrinsic(num: u32) -> u32 {
        let mut num = num;
        num ^= num >> 16;
        num ^= num >> 8;
        num ^= num >> 4;
        num ^= num >> 2;
        num ^= num >> 1;
        num
    }
}
