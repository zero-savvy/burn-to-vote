use crate::circuits::{Circuit, CircuitIdentifier};
use primitive_types::U256;

pub struct NullifierCircuit {
    identifier: CircuitIdentifier<'static>,
    private_key: U256,
    blinding_factor: u64,
    ceremony_id: u64,
    nullifier: String,
}

impl NullifierCircuit {
    pub fn new(
        private_key: U256,
        blinding_factor: u64,
        ceremony_id: u64,
        nullifier: String,
    ) -> Self {
        Self {
            identifier: CircuitIdentifier {
                circuit_name: "nullifier",
            },
            private_key,
            blinding_factor,
            ceremony_id,
            nullifier,
        }
    }
    pub fn format_inputs(&self) -> Result<String, Box<dyn std::error::Error>> {
        let inputs = format!(
            "{{\"privateKey\": \"{}\",
            \"blindingFactor\": \"{}\",
            \"ceremonyID\": \"{}\",
            \"nullifier\": \"{}\" }}",
            self.private_key.to_string(),
            self.blinding_factor.to_string(),
            self.ceremony_id.to_string(),
            U256::from_str_radix(&self.nullifier[2..], 16)
                .unwrap()
                .to_string()
        );
        Ok(inputs)
    }
}

impl Circuit for NullifierCircuit {
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
