//! Morton encoding using clmul
//!
//! See <https://en.wikipedia.org/wiki/Z-order_curve>

use crate::clmul;

/// morton_encode 2 binary numbers
///
/// * x - number will be dilated and placed in least significant position ie. bit 0
/// * y - number will be dilated and placed one bit to left, ie. bit 1
///
/// # Returns
///
/// morton/z-order encoded number
#[inline]
pub fn morton_encode(x: u64, y: u64) -> u128 {
    let xd = dilate(x);
    let yd = dilate(y);
    (yd << 1) | xd
}

/// Dilate a binary number
///
/// The dilation is implemented by executing: clmul(x,x)
///
/// * x - number will be dilated and placed in least significant position ie. bit 0
///
/// # Returns
///
/// The dilated number, the binary bits of the original will have binary zeros interspersed between them.

#[inline]
pub fn dilate(x: u64) -> u128 {
    clmul(x, x)
}

#[inline]
pub fn morton_encode_magic(x: u64, y: u64) -> u64 {
    let xd = dilate_magic(x);
    let yd = dilate_magic(y);
    (yd << 1) | xd
}

/// Dilate a binary number
///
/// The dilation is implemented by executing: clmul(x,x)
///
/// * x - number will be dilated and placed in least significant position ie. bit 0
///
/// # Returns
///
/// The dilated number, the binary bits of the original will have binary zeros interspersed between them.

#[inline]
pub fn dilate_magic(x: u64) -> u64 {
    //really u16->u32
    let x = (x | (x << 8)) & 0x00FF00FF;
    let x = (x | (x << 4)) & 0x0F0F0F0F;
    let x = (x | (x << 2)) & 0x0F0F0F0F;
    let x = (x | (x << 2)) & 0x55555555;

    // let mut x=x & 0x1fffff; //isolate 21 bits
    // x = (x | x << 32) & 0x1f00000000ffff;
    // x = (x | x << 16) & 0x1f0000ff0000ff;
    // x = (x | x << 8) & 0x100f00f00f00f00f;
    // x = (x | x << 4) & 0x10c30c30c30c30c3;
    // x = (x | x << 2) & 0x1249249249249249;
    x
}

//TODO  dilate3
// inline uint64_t splitBy3(unsigned int a){
// uint64_t x = a & 0x1fffff; // we only look at the first 21 bits
// x = (x | x << 32) & 0x1f00000000ffff; // shift left 32 bits, OR with self, and 00011111000000000000000000000000000000001111111111111111
// x = (x | x << 16) & 0x1f0000ff0000ff; // shift left 32 bits, OR with self, and 00011111000000000000000011111111000000000000000011111111
// x = (x | x << 8) & 0x100f00f00f00f00f; // shift left 32 bits, OR with self, and 0001000000001111000000001111000000001111000000001111000000000000
// x = (x | x << 4) & 0x10c30c30c30c30c3; // shift left 32 bits, OR with self, and 0001000011000011000011000011000011000011000011000011000100000000
// x = (x | x << 2) & 0x1249249249249249;
// return x;
// }

//x | (y << 1) | (z << 2)

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_dilate() {
        let input = 0b1101;
        let expected = 0b1010001;
        let res = dilate(input);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_morton_encode() {
        let x = 543;
        let y = 23765;
        let res = morton_encode(x, y);
        let res_magic = morton_encode_magic(x, y);
        println!("X {:032b}", x);
        println!("Y {:032b}", y);
        println!("R {:032b}", res);
        println!("M {:032b}", res_magic);

        let roundtrip: [u32; 2] = morton_encoding::morton_decode(res as u64);
        println!("Rt {:?}", roundtrip);

        //https://graphics.stanford.edu/~seander/bithacks.html#InterleaveTableObvious
    }

    #[test]
    fn test_morton_decode() {
        use morton_encoding::{morton_decode, morton_encode};

        let input = 992;
        let output: [u8; 5] = morton_decode(input);
        assert_eq!(output, [2u8; 5]);
        let input = [543u32, 23765];
        let encoded_input: u64 = morton_encode(input);
        let reassembled_input: [u32; 2] = morton_decode(encoded_input);
        assert_eq!(input, reassembled_input);
    }

    #[test]
    fn test_decode() {
        let b = [0x55555555u32, 0x33333333, 0x0F0F0F0F, 0x00FF00FF];

        let mut x = 0b1111_1111_1111_1111;
        for (i, b) in b.iter().enumerate().rev() {
            let s = 1 << i;
            println!("{}: s={}", i, s);
            println!("x=     {:032b}", x);
            println!("x<<s=  {:032b}", x << s);
            println!("x|x<<s={:032b}", (x << s) | x);
            println!("b=     {:032b}", b);
            let x = (x | (x << s)) & b;
            println!("x'=    {:032b}", x);
        }
    }
}
