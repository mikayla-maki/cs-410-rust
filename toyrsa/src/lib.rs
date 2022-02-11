use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

///Computes 'Carmichael's totient function', the least common
///  multiple of p - 1 and q - 1.
fn lambda(p: u64, q: u64) -> u64 {
    lcm(p - 1, q - 1)
}

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    loop {
        let p = rsa_prime();
        let q = rsa_prime();
        let l = lambda(p as u64, q as u64);
        if EXP <= l && gcd(EXP, l) == 1 {
            return (p, q);
        }
    }
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d = modinverse(EXP, lambda(key.0 as u64, key.1 as u64));
    //As the type of the message field in `encrypt()` was u32,
    // this is guaranteed to be a safe demotion
    u32::try_from(modexp(msg, d, key.0 as u64 * key.1 as u64)).unwrap()
}
