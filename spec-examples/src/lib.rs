pub mod aes_gcm;
pub mod blake2;
pub mod chacha20_poly1305;
pub mod curve25519;
pub mod fips202;
pub mod sha2;
pub mod hkdf;
pub mod hmac;
pub mod p256;

use hacspec::prelude::*;
use hacspec_derive::*;

#[allow(dead_code)]
#[derive(Default, Clone, Debug, Numeric)]
struct NumericPair {
    fst: ByteSeq,
    snd: ByteSeq
}
