#![warn(dead_code)]
#[macro_use]
use crate::circuits::{Circuit, CircuitIdentifier};
use primitive_types::U256;

pub struct BurnAddressCircuit {
    pub identifier: CircuitIdentifier<'static>,
    pub address: String,
    pub private_key: U256,
    pub blinding_factor: u64,
    pub ceremony_id: u64,
    pub random_secret: u64,
    pub vote: u64,
}

impl BurnAddressCircuit {
    pub fn new(
        address: String,
        private_key: U256,
        blinding_factor: u64,
        ceremony_id: u64,
        random_secret: u64,
        vote: u64,
    ) -> Self {
        Self {
            identifier: CircuitIdentifier {
                circuit_name: "burnAddress",
            },
            address,
            private_key,
            blinding_factor,
            ceremony_id,
            random_secret,
            vote,
        }
    }
    pub fn format_inputs(&self) -> Result<String, Box<dyn std::error::Error>> {
        let inputs = format!(
            "{{ \"address\": \"{}\",
            \"privateKey\": \"{}\",
            \"blinding_factor\": \"{}\",
            \"ceremonyID\": \"{}\",
            \"personalID\": \"{}\",
            \"vote\": \"{}\" }}",
            U256::from_str_radix(&self.address[2..], 16)
                .unwrap()
                .to_string(),
            self.private_key.to_string(),
            serde_json::to_string(&self.blinding_factor).unwrap(),
            serde_json::to_string(&self.ceremony_id).unwrap(),
            serde_json::to_string(&self.random_secret).unwrap(),
            serde_json::to_string(&self.vote).unwrap()
        );
        Ok(inputs)
    }
}

impl Circuit for BurnAddressCircuit {
    fn generate_input_file(&self, inputs: String) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_input_file(inputs)?;
        Ok(())
    }

    fn generate_witness(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_witness()?;
        Ok(())
    }

    fn setup_zkey(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.setup_zkey()?;
        Ok(())
    }

    fn generate_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_proof()?;

        Ok(())
    }

    fn setup_vkey(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.setup_vkey()?;
        Ok(())
    }

    fn verify_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.verify_proof()?;
        Ok(())
    }
    fn generate_verifier(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.identifier.generate_verifier()?;
        Ok(())
    }
}
