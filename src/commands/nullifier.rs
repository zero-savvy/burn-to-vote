use crate::utils::config::*;
use crate::utils::u256_to_fp;
use ff::PrimeField;
use log::info;
use poseidon_rs::{Fr, FrRepr, Poseidon};
use primitive_types::U256;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Nullifier {
    pub private_key: String,
    pub ceremony_id: u64,
    pub blinding_factor: u64,
}

pub async fn generate_nullifier(data: Config, blinding_factor: u64, private_key: String) -> String {
    info!("generaring nullifier ..");

    let private_key = U256::from_str_radix(&private_key, 16).unwrap();
    let private_key_fp = u256_to_fp(private_key);

    let blinding_factor_fp = Fr::from_repr(FrRepr::from(blinding_factor)).unwrap();
    let ceremony_id_fp = Fr::from_repr(FrRepr::from(data.ceremony_id.unwrap())).unwrap();

    let input: Vec<Fr> = vec![private_key_fp, ceremony_id_fp, blinding_factor_fp];

    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input).unwrap();

    let hash_string = hash.into_repr().to_string();

    hash_string
}

// TODO: complete tests
#[cfg(test)]
mod tests {

    use crate::commands::burn_address;

    use super::*;
    #[tokio::test]
    async fn test_nullifier_generation() {
        let mock_config = Config::mock_config().await;
        let blinding_factor = rand::random::<u64>();
        let pk = rand::random::<u64>().to_string();
        let nullifier = generate_nullifier(mock_config.clone(), blinding_factor, pk.clone()).await;
        let nullifier_clone = generate_nullifier(mock_config, blinding_factor, pk).await;

        assert_eq!(nullifier, nullifier_clone);
    }
}
