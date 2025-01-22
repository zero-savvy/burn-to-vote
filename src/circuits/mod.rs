pub mod burn_address;
pub mod nullifier;

pub trait Circuit {
    fn generate_inputs(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn generate_witness(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn setup_zkey(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn setup_vkey(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn generate_proof(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn verify_proof(&self) -> Result<(), Box<dyn std::error::Error>>;
}
