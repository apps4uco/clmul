
Crate to call the clmul instruction

Otherwise known as:
* Carry-Less Multiplication
* Carry-Less Multiply
* Carry-Less Product


# TL;DR;

Its an operation that does a multiply of 2 integers using XOR to "add" the partial results.

Current version: 0.5.0

License: MIT OR Apache-2.0

## Use

To quote
[RISC-V Bitmanip Extension V0.90 23](https://raw.githubusercontent.com/riscv/riscv-bitmanip/master/bitmanip-0.90.pdf):

The classic applications for clmul are CRC [11, 22] and GCM, but more applications exist, including
the following examples.
There are obvious applications in hashing and pseudo random number generations. For exam-
ple, it has been reported that hashes based on carry-less multiplications can outperform Google’s
CityHash [15].
clmul of a number with itself inserts zeroes between each input bit. This can be useful for generating
Morton code [21].
clmul of a number with -1 calculates the prefix XOR operation. This can be useful for decoding
gray codes.
Another application of XOR prefix sums calculated with clmul is branchless tracking of quoted
strings in high-performance parsers. [14]
Carry-less multiply can also be used to implement Erasure code efficiently. [12]



# Wikipedia 

<https://en.wikipedia.org/wiki/CLMUL_instruction_set>


Carry-less multiplication is polynomial multiplication, where each bit in a register is the coefficient of a polynomial over GF(2).


# RiscV


[RISC-V Bitmanip Extension V0.90 23](https://raw.githubusercontent.com/riscv/riscv-bitmanip/master/bitmanip-0.90.pdf)

2.4 Carry-less multiply (clmul, clmulh, clmulr)

"That means clmulh
is equivalent to clmulr followed by a 1-bit right shift. (The MSB of a clmulh result is always zero.)
Another equivalent definition of clmulr is clmulr(a,b) := rev(clmul(rev(a), rev(b))). (The
“r” in clmulr means reversed.)"

<https://raw.githubusercontent.com/riscv/riscv-bitmanip/master/bitmanip-draft.pdf>

# LLVM

<https://github.com/llvm-mirror/clang/blob/master/lib/Headers/__wmmintrin_pclmul.h#L26>

# RFC to include clmul in LLVM intrinsics

https://discourse.llvm.org/t/rfc-carry-less-multiplication-instruction/1347

# CRC

http://www.ross.net/crc/download/crc_v3.txt

# Barrett Reduction

https://www.corsix.org/content/barrett-reduction-polynomials

# Intel

<https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=clmul>




# Rust

# Issue Implement CLMUL instruction set

<https://github.com/rust-lang/stdarch/issues/318>

Implemented in

## core::core_arch::x86::pclmulqdq::_mm_clmulepi64_si128

pub unsafe fn _mm_clmulepi64_si128(
    a: __m128i,
    b: __m128i,
    const IMM8: i32
) -> __m128i

# Compiler flags

 //--cfg polyval_armv8
        //intel CMLMUL
        // On aarch64 targets including aarch64-apple-darwin (Apple M1) and Linux targets such as aarch64-unknown-linux-gnu and aarch64-unknown-linux-musl, support for using the PMULL instructions in ARMv8’s Cryptography Extensions with the following RUSTFLAGS:
        //
        // --cfg polyval_armv8
        //
        // On Linux and macOS when the ARMv8 features are enabled, support for PMULL
        // intrinsics is autodetected at runtime. On other platforms the crypto target feature must be enabled via RUSTFLAGS.

        // RUSTFLAGS="-Ctarget-cpu=sandybridge" cargo bench
        //core::arch::aarch64::poly8x8_t

# Benchmarks

<https://stackoverflow.com/questions/39490345/interleave-bits-efficiently>

CLMUL is great on Broadwell and later (1 uop, 1c throughput), or 3 uops (2c throughput) for the YMM/ZMM version on Ice Lake. Then down to 1 uop again for the YMM/ZMM version on Alder Lake. On AMD, it's 4 uops (2c throughput not fully pipelined) for the XMM version on Zen1/2/3 (and the YMM version which is new in Zen3). uops.info. But pdep is disastrously slow on Zen 1/2, only having proper hw support in Zen3. –

<https://encode.su/threads/3760-clmul-instruction-for-interleaving-bits-(For-improve-cache-speed)>

CLMUL is 6-7clk, so I doubt that it could be useful as a hash function.


<https://stackoverflow.com/questions/53401547/is-clmul-constant-time>

Numbers from Agner Fog's tables

On Intel since Broadwell, pclmuludq is 1 uop. On Skylake, it's 7 cycle latency, 1 per clock throughput. (So you need to keep 7 independent PCLMUL operations in flight to saturate the execution unit on port 5). Broadwell has 5 cycle latency. With a memory source operand, it's 1 extra uop.

On Haswell, it's 3 uops (2p0 p5) with 7 cycle latency and one per 2 clock throughput.

On Sandybridge/IvyBridge it's 18 uops, 14c latency, one per 8 clock throughput.

On Westmere (2nd Gen Nehalem) it's 12c latency, one per 8c throughput. (Unknown number of uops, neither Agner Fog nor uops.info has it. But we can safely assume it's microcoded.) This was the first generation to support the instruction- one of the very few differences from Nehalem to Westmere.

On Ryzen it's 4 uops, 4c latency, one per 2 clock throughput. http://instlatx64.atw.hu/ shows it 4.5 cycle latency. I'm not sure what the difference is between their testing and Agner's.

On Piledriver it's 5 uops, 12c latency, one per 7 clock throughput.

On Jaguar it's 1 uop, 3c latency, one per 1 clock throughput!

On Silvermont it's 8 uops, 10c latency/throughput. Goldmont = 3 uops, 6c lat / 3c tput.
