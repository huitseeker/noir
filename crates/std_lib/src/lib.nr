mod hash;
mod array;
mod merkle;
mod schnorr;
mod ecdsa_secp256k1;
mod scalar_mul;

#[builtin(set_pub)]
fn set_as_public(_input : Field) {}