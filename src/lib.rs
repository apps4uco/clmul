//#![no_std] TODO remove println from tests
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Crate to call the clmul instruction
//!
//! Otherwise known as:
//! * Carry-Less Multiplication
//! * Carry-Less Multiply
//! * Carry-Less Product
//!
//! Maybe one day this functionality will be present in the core or std Rust libraries.

/**
 * clmul
 *
 *
 */
#[inline]
pub fn clmul(a: u64, b: u64) -> u128 {
    //Intel's PCLMULQDQ instruction (part of the CLMUL extension,

    #[cfg(all(
        target_arch = "aarch64",
        target_feature = "neon",
        target_feature = "aes"
    ))]
    return clmul_aarch64_neon(a, b);

    #[cfg(target_arch = "x86_64")]
    {
        if core_detect::is_x86_feature_detected!("pclmulqdq") {
            return unsafe { clmul_intel(a, b)};
        }
    }

    #[allow(unreachable_code)]
    return clmul_nosimd(a, b);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
#[inline]
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
fn clmul_intel(a: u64, b: u64) -> u128 {
    use core::arch::x86_64::*;
    // SAFETY: target_features "x86_64" and "pclmulqdq" are available in this function.
    unsafe {
        core::mem::transmute(_mm_clmulepi64_si128(
            _mm_cvtsi64_si128(a as i64),
            _mm_cvtsi64_si128(b as i64),
            0,
        ))
    }
}

// Implementation of clmul using instrinsics (vmull_p64) on Arm Processor with AES
#[cfg(all(
    target_arch = "aarch64",
    target_feature = "neon",
    target_feature = "aes"
))]
#[inline]
fn clmul_aarch64_neon(a: u64, b: u64) -> u128 {
    // SAFETY: target_features "neon" and "aes" are available in this function.
    unsafe { core::arch::aarch64::vmull_p64(a, b) }
}

// Fallback implementation
#[inline]
#[allow(dead_code)]
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
    fn test_clmul_u8() {
        let result = clmul(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_clmul_u32() {
        let result = clmul(u32::MAX as u64, u32::MAX as u64);
        assert_eq!(result, 6148914691236517205);
    }

    #[test]
    fn test_clmul_u64() {
        let result = clmul(u64::MAX as u64, u64::MAX as u64);
        assert_eq!(result, 113427455640312821154458202477256070485);
    }
}
