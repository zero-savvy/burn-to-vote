use crate::circuits::{Circuit, CircuitIdentifier};
use primitive_types::U256;
pub struct BurnAddressCircuit {
    identifier: CircuitIdentifier<'static>,
    address: String,
    private_key: U256,
    blinding_factor: u64,
    ceremony_id: u64,
    personal_id: u64,
    vote: u64,
}

impl BurnAddressCircuit {
    pub fn new(
        address: String,
        private_key: U256,
        blinding_factor: u64,
        ceremony_id: u64,
        personal_id: u64,
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
            personal_id,
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
            serde_json::to_string(&self.personal_id).unwrap(),
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
}
