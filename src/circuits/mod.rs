pub mod burn_address_c;
pub mod merkle_tree_c;
pub mod mpt_c;
pub mod nullifier_c;
pub mod vote_c;
use log::{error, info};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub trait Circuit {
    fn generate_input_file(&self, inputs: String) -> Result<(), Box<dyn Error>>;
    fn generate_witness(&self) -> Result<(), Box<dyn Error>>;
    fn setup_zkey(&self) -> Result<(), Box<dyn Error>>;
    fn setup_vkey(&self) -> Result<(), Box<dyn Error>>;
    fn generate_proof(&self) -> Result<(), Box<dyn Error>>;
    fn verify_proof(&self) -> Result<(), Box<dyn Error>>;
    fn generate_verifier(&self) -> Result<(), Box<dyn Error>>;
}

pub struct CircuitIdentifier<'a> {
    pub circuit_name: &'a str,
}

impl<'a> Circuit for CircuitIdentifier<'a> {
    fn generate_input_file(&self, inputs: String) -> Result<(), Box<dyn Error>> {
        info!("Generating {:?} circuits inputs ...", self.circuit_name);
        fs::create_dir_all("circuits/inputs")?;
        let inputs_path = format!("circuits/inputs/{}.json", self.circuit_name);
        let mut file = File::create(inputs_path)?;
        file.write_all(inputs.as_bytes())?;
        info!("Inputs generated successfully.");
        Ok(())
    }

    fn generate_witness(&self) -> Result<(), Box<dyn Error>> {
        info!("Generating {:?} circuits witness ...", self.circuit_name);

        let calculate_command = format!(
            "snarkjs wtns calculate circuits/{name}/{name}_js/{name}.wasm circuits/inputs/{name}.json circuits/{name}/witness.wtns",
            name = self.circuit_name
        );
        match self.run_command(&calculate_command) {
            Ok(_) => info!("generate_witness Command ran successfully!"),
            Err(e) => error!("generate_witness Command failed: {}", e),
        }

        let export_witness = format!(
            "snarkjs wtns export json circuits/{name}/witness.wtns circuits/{name}/witness.json",
            name = self.circuit_name
        );
        match self.run_command(&export_witness) {
            Ok(_) => info!("export_witness Command ran successfully!"),
            Err(e) => error!("generate_witness Command failed: {}", e),
        }

        info!("Witness generated successfully.");
        Ok(())
    }

    fn setup_zkey(&self) -> Result<(), Box<dyn Error>> {
        info!("Setting up {:?} circuits zkey ...", self.circuit_name);
        info!("This could take a while ...");

        let setup_command = format!(
            "snarkjs groth16 setup circuits/{name}/{name}.r1cs circuits/setup/pot_final.ptau circuits/{name}/{name}_0000.zkey",
            name = self.circuit_name
        );
        match self.run_command(&setup_command) {
            Ok(_) => info!("Setup_zkey Command ran successfully!"),
            Err(e) => error!("Setup_zkey Command failed: {}", e),
        }

        Ok(())
    }

    fn setup_vkey(&self) -> Result<(), Box<dyn Error>> {
        info!("Setting up {:?} circuits vkey ...", self.circuit_name);
        info!("This could take a while ...");

        let vkey_command = format!(
            "snarkjs zkey export verificationkey circuits/{name}/{name}_0000.zkey circuits/{name}/verification_key.json",
            name = self.circuit_name
        );
        match self.run_command(&vkey_command) {
            Ok(_) => info!("Setup_vkey Command ran successfully!"),
            Err(e) => error!("Setup_vkey Command failed: {}", e),
        }

        Ok(())
    }

    fn generate_proof(&self) -> Result<(), Box<dyn Error>> {
        info!("Generating {:?} proof ...", self.circuit_name);

        fs::create_dir_all("circuits/proofs")?;
        let proof_command = format!(
            "snarkjs groth16 prove circuits/{name}/{name}_0000.zkey circuits/{name}/witness.wtns circuits/proofs/{name}_proof.json circuits/proofs/{name}_public.json",
            name = self.circuit_name
        );

        match self.run_command(&proof_command) {
            Ok(_) => info!("generate_proof Command ran successfully!"),
            Err(e) => error!("generate_proof Command failed: {}", e),
        }

        Ok(())
    }

    fn verify_proof(&self) -> Result<(), Box<dyn Error>> {
        info!("Verifying {:?} circuits proof ...", self.circuit_name);

        let verify_command = format!(
            "snarkjs groth16 verify circuits/{name}/verification_key.json circuits/proofs/{name}_public.json circuits/proofs/{name}_proof.json",
            name = self.circuit_name
        );
        match self.run_command(&verify_command) {
            Ok(_) => info!("verify_proof Command ran successfully!"),
            Err(e) => error!("verify_proof Command failed: {}", e),
        }

        Ok(())
    }

    fn generate_verifier(&self) -> Result<(), Box<dyn Error>> {
        info!(
            "Generating {:?} circuits verifier contract ...",
            self.circuit_name
        );

        let verify_command = format!(
            "snarkjs zkey export solidityverifier circuits/{name}/{name}_0000.zkey circuits/{name}/{name}_verifier.sol",
            name = self.circuit_name
        );
        match self.run_command(&verify_command) {
            Ok(_) => info!("generate_verifier Command ran successfully!"),
            Err(e) => error!("generate_verifier Command failed: {}", e),
        }

        Ok(())
    }
}

impl<'a> CircuitIdentifier<'a> {
    fn run_command(&self, command: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("sh").arg("-c").arg(command).output()?;
        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command failed: {}", stdout),
            )));
        }
        Ok(())
    }
}
