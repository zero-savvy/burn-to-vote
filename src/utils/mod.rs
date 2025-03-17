pub mod mt;
pub mod account;
use alloy::primitives::Address;
use ff::PrimeField;
use num_bigint::BigUint;
use num_traits::Num;
use poseidon_rs::{Fr, FrRepr};
use primitive_types::U256;

pub fn fr_repr_to_bytes(fr_repr: &FrRepr) -> [u8; 32] {
    let mut bytes: [u8; 32] = unsafe { std::mem::transmute(*fr_repr) };
    bytes.reverse();
    bytes
}

pub fn u256_to_fp(pk: U256) -> Fr {
    let modulus: BigUint = BigUint::from_str_radix(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617",
        10,
    )
    .unwrap();
    let pk_biguint = BigUint::from_bytes_le(&pk.to_little_endian());
    let reduced_biguint = pk_biguint % modulus;
    let mut repr_bytes = [0u8; 32];
    let reduced_bytes = reduced_biguint.to_bytes_le();
    repr_bytes[..reduced_bytes.len()].copy_from_slice(&reduced_bytes);

    let mut u64s = [0u64; 4];
    for (i, chunk) in repr_bytes.chunks(8).enumerate() {
        u64s[i] = u64::from_le_bytes(chunk.try_into().unwrap());
    }
    let repr = FrRepr(u64s);
    Fr::from_repr(repr).unwrap()
}

pub fn address_to_fr(address: Address) -> Fr {
    let bytes = address.into_word();
    let u256 = U256::from_big_endian(&bytes[..]);
    Fr::from_str(&u256.to_string()).unwrap()
}

