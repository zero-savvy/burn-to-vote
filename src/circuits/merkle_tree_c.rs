use crate::circuits::{Circuit, CircuitIdentifier};
use crate::utils::mt::Proof;
use ff::PrimeField;

pub struct MtCircuit {
    identifier: CircuitIdentifier<'static>,
    node: Proof,
}

impl MtCircuit {
    pub fn new(node: Proof) -> Self {
        Self {
            identifier: CircuitIdentifier {
                circuit_name: "merkleTree",
            },
            node,
        }
    }
    pub fn format_inputs(&self) -> Result<String, Box<dyn std::error::Error>> {
        let path_elements: Vec<String> = self
            .node
            .pathElements
            .iter()
            .map(|p| p.into_repr().to_string())
            .collect();

        let inputs = serde_json::json!({
            "root": self.node.root.into_repr().to_string(),
            "leaf": self.node.leaf.into_repr().to_string(),
            "pathElements": path_elements,
            "pathIndices": self.node.pathIndices,
        });
        Ok(inputs.to_string())
    }
}

impl Circuit for MtCircuit {
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
