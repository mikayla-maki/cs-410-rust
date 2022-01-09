use toy_rsa_lib::*;

/*

    E = 65537

    𝜆(p, q):
        return least common multiple of p - 1 and q - 1

    encrypt(key, msg):
        return msg^E mod key

    decrypt(key = p ⋅ q, msg):
        d ← inverse of E mod 𝜆(p, q)
        return msg^d mod (p ⋅ q)

    genkey:
        repeat 
            p, q ← rsa primes (primes in range 2^31 .. 2^32-1)
        until E < 𝜆(p, q) and gcd(E, 𝜆(p, q)) = 1
        return p, q
*/

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    (0, 0)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    0
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    0
}
