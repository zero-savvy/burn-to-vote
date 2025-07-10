use crate::circuits::burn_address_c::*;
// use crate::circuits::Circuit;
use crate::utils::{config::*, fr_repr_to_bytes, u256_to_fp};
use ethers::types::{Address, BigEndianHash, H256};
use ff::PrimeField;
use log::info;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256;
use sha2::digest::block_buffer;
use structopt::StructOpt;
use ethers::providers::{Http, Middleware, Provider};
type PrimitiveU256 = primitive_types::U256;

#[derive(Debug, StructOpt, Clone)]
pub struct BurnAddress {
    pub private_key: String,
    pub ceremony_id: u64,
    pub blinding_factor: u64,
    pub votingBlock: u64,
    pub action_value: u64,
}

pub async fn burn_address(
    config: Config,
    private_key: String,
    blinding_factor: u64,
    staring_block: u64,
    action_value: u64,
) -> (BurnAddressCircuit, Address, String) {
    info!("Genrating burn address ...");
    let provider: Provider<Http> = Provider::<Http>::try_from(config.network.url()).expect("failed to run provider.").clone();

    let block = provider.get_block(staring_block).await.expect("failed to get block number.");
    let mut block_hash_u256: PrimitiveU256 = U256::zero();
    if let Some(block) = block {
        let block_hash = block.hash.unwrap();
        let block_hash_u256 =  block_hash.into_uint();

    }
    let private_key = U256::from_str_radix(&private_key.clone(), 16).unwrap();

    let private_key_fp = u256_to_fp(private_key);

    let ceremony_id_fp = Fr::from_repr(FrRepr::from(config.ceremony_id.unwrap())).unwrap();
    let blinding_factor_fp = Fr::from_repr(FrRepr::from(blinding_factor)).unwrap();
    let random_secret_fp = Fr::from_repr(FrRepr::from(blinding_factor)).unwrap();
    let block_hash_fp= u256_to_fp(block_hash_u256);

    let action_fp = Fr::from_repr(FrRepr::from(action_value)).unwrap();

    let input: Vec<Fr> = vec![
        private_key_fp,
        ceremony_id_fp,
        blinding_factor_fp,
        random_secret_fp,
        block_hash_fp,
        action_fp,
    ];

    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input).unwrap();
    let rep = hash.into_repr();
    let rep_str = hash.into_repr().to_string();
    let bytes = fr_repr_to_bytes(&rep);
    let address_bytes = &bytes[12..];
    let address = Address::from_slice(address_bytes);

    let circuit = BurnAddressCircuit::new(
        rep_str.clone(),
        private_key,
        blinding_factor,
        config.ceremony_id.unwrap(),
        blinding_factor,
        block_hash_u256,
        action_value,
    );

    (circuit, address, block_hash_u256.to_string())
}

// TODO: complete tests
// #[cfg(test)]
// mod tests {

//     use crate::commands::burn_address;

//     use super::*;
//     #[tokio::test]
//     async fn test_burn_address_generation() {
//         let mock_config = Config::mock_config().await;
//         let blinding_factor = rand::random::<u64>();
//         let pk = rand::random::<u64>().to_string();
//         let (_, ba) = burn_address(mock_config.clone(), pk.clone(), blinding_factor, 1).await;
//         let (_, ba_clone) = burn_address(mock_config, pk, blinding_factor, 1).await;

//         assert_eq!(ba, ba_clone);
//     }
// }
