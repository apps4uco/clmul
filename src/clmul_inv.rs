//https://bitmath.blogspot.com/2013/05/carryless-multiplicative-inverse.html

// static uint clmulinv(uint x)
// {
// uint inv = 1;
// uint rem = x;
// for (int i = 1; i < 32; i++)
// {
// if (((rem >> i) & 1) != 0)
// {
// rem ^= x << i;
// inv |= 1u << i;
// }
// }
// return inv;
// }
//
//
// A variation of the algorithm to find a multiplicative inverse modulo a power of two (see inv here) also works, which is useful when clmul is fast:
//
// static uint clmulinv(uint d)
// {
// uint x = 1;
// for (int i = 0; i < 5; i++)
// {
// x = clmul(x, clmul(x, d));
// }
// return x;
// }
